//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2935/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2935(t51913: f64, t51915: f64, t63240: f64, t63242: f64, t77663: f64, t77667: f64, t77670: f64, t77672: f64, t77674: f64, t77676: f64, t77679: f64, t41592: f64, t77499: f64, t77503: f64, t77505: f64, t77683: f64, t77686: f64, t77688: f64, t77690: f64, t77692: f64, t77695: f64, t77698: f64, t77700: f64) -> (f64, f64) {
    let t77998 = 0.54771111111111111112e0_f64 * t51913 - 0.91285185185185185187e-1_f64 * t51915 - 0.10954222222222222222e0_f64 * t77663 + 0.98587999999999999998e0_f64 * t63240 - 0.65725333333333333332e0_f64 * t63242 + 0.2434271604938271605e-1_f64 * t77667 - 0.82156666666666666667e-1_f64 * t77670 - 0.230371875e0_f64 * t77672 + 0.46074375e0_f64 * t77674 + 0.46074375e0_f64 * t77676 - 0.9494625e0_f64 * t77679;
    let t78010 = 0.15358125e0_f64 * t77683 - 0.3560484375e1_f64 * t77686 + 0.427258125e1_f64 * t77688 - 0.28483875e1_f64 * t77690 - 0.28483875e1_f64 * t77692 + 0.1151859375e0_f64 * t77695 + t41592 + 0.427258125e1_f64 * t77698 - 0.230371875e0_f64 * t77700 + 0.11072839506172839506e0_f64 * t77499 - 0.29896666666666666667e0_f64 * t77503 + 0.99655555555555555557e-1_f64 * t77505;
    (t77998, t78010)
}
