//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2935/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2935<F: Float>(t51913: F, t51915: F, t63240: F, t63242: F, t77663: F, t77667: F, t77670: F, t77672: F, t77674: F, t77676: F, t77679: F, t41592: F, t77499: F, t77503: F, t77505: F, t77683: F, t77686: F, t77688: F, t77690: F, t77692: F, t77695: F, t77698: F, t77700: F) -> (F, F) {
    let t77998 = F::cast_from(0.54771111111111111112e0_f64) * t51913 - F::cast_from(0.91285185185185185187e-1_f64) * t51915 - F::cast_from(0.10954222222222222222e0_f64) * t77663 + F::cast_from(0.98587999999999999998e0_f64) * t63240 - F::cast_from(0.65725333333333333332e0_f64) * t63242 + F::cast_from(0.2434271604938271605e-1_f64) * t77667 - F::cast_from(0.82156666666666666667e-1_f64) * t77670 - F::cast_from(0.230371875e0_f64) * t77672 + F::cast_from(0.46074375e0_f64) * t77674 + F::cast_from(0.46074375e0_f64) * t77676 - F::cast_from(0.9494625e0_f64) * t77679;
    let t78010 = F::cast_from(0.15358125e0_f64) * t77683 - F::cast_from(0.3560484375e1_f64) * t77686 + F::cast_from(0.427258125e1_f64) * t77688 - F::cast_from(0.28483875e1_f64) * t77690 - F::cast_from(0.28483875e1_f64) * t77692 + F::cast_from(0.1151859375e0_f64) * t77695 + t41592 + F::cast_from(0.427258125e1_f64) * t77698 - F::cast_from(0.230371875e0_f64) * t77700 + F::cast_from(0.11072839506172839506e0_f64) * t77499 - F::cast_from(0.29896666666666666667e0_f64) * t77503 + F::cast_from(0.99655555555555555557e-1_f64) * t77505;
    (t77998, t78010)
}
