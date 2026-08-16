//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1618/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1618(t157: f64, t190: f64, t87145: f64, t49926: f64, t49940: f64, t76972: f64, t61165: f64, t39756: f64, t39760: f64, t39764: f64, t39770: f64, t39773: f64, t39783: f64, t39786: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t87640 = 24.0_f64 * t87145 * t157 * t190;
    let t87641 = 0.86748650402413918736e-1_f64 * t49926;
    let t87642 = 0.14035736694323150897e2_f64 * t49940;
    let t87643 = 0.73245789224026180216e-3_f64 * t76972;
    let t87644 = 72.0_f64 * t61165;
    let t87645 = t39756 + t39760 - t39764 + t87640 + t39770 - t87641 + t87642 + t39773 - t87643 + t87644 - t39783 - t39786;
    (t87640, t87641, t87642, t87643, t87644, t87645)
}
