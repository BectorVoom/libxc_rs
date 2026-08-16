//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1670/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1670(t52128: f64, t63453: f64, t63459: f64, t63464: f64, t63533: f64, t63538: f64, t63545: f64, t77559: f64, t77561: f64, t77806: f64, t77858: f64, t88252: f64, t88257: f64, t88260: f64) -> f64 {
    let t88336 = 0.27785333333333333333e0_f64 * t77806 + 0.12349037037037037037e1_f64 * t52128 - 0.91817777777777777776e0_f64 * t63453 + 0.27545333333333333333e1_f64 * t63459 - 0.23154444444444444445e0_f64 * t63533 + 0.13892666666666666667e1_f64 * t63538 - 0.69463333333333333334e0_f64 * t63545 + 0.13772666666666666667e1_f64 * t77559 - 0.41318e1_f64 * t77561 + 0.6311625e0_f64 * t88252 - 0.13772666666666666666e1_f64 * t63464 + 0.27785333333333333333e0_f64 * t77858 + 0.125034e1_f64 * t88257 - 0.375102e1_f64 * t88260;
    t88336
}
