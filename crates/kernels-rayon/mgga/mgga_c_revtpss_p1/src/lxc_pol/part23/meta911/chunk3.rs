//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2929/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2929(t52546: f64, t52547: f64, t63240: f64, t63242: f64, t77663: f64, t77667: f64, t77670: f64, t77672: f64, t77674: f64, t77676: f64, t77679: f64, t41672: f64, t77499: f64, t77503: f64, t77505: f64, t77683: f64, t77686: f64, t77688: f64, t77690: f64, t77692: f64, t77695: f64, t77698: f64, t77700: f64) -> (f64, f64) {
    let t77886 = t52546 - t52547 - 0.13892666666666666667e0_f64 * t77663 + 0.125034e1_f64 * t63240 - 0.83356000000000000002e0_f64 * t63242 + 0.30872592592592592593e-1_f64 * t77667 - 0.104195e0_f64 * t77670 - 0.473371875e0_f64 * t77672 + 0.94674375e0_f64 * t77674 + 0.94674375e0_f64 * t77676 - 0.17648625e1_f64 * t77679;
    let t77898 = 0.31558125e0_f64 * t77683 - 0.6618234375e1_f64 * t77686 + 0.794188125e1_f64 * t77688 - 0.52945875e1_f64 * t77690 - 0.52945875e1_f64 * t77692 + 0.2366859375e0_f64 * t77695 + t41672 + 0.794188125e1_f64 * t77698 - 0.473371875e0_f64 * t77700 + 0.19128703703703703704e0_f64 * t77499 - 0.516475e0_f64 * t77503 + 0.17215833333333333333e0_f64 * t77505;
    (t77886, t77898)
}
