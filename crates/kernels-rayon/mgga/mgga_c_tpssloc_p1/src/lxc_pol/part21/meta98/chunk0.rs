//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 686/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk686(t40: f64, t2244: f64, t2250: f64, t2433: f64, t73: f64, t197: f64, zeta_threshold: f64) -> (f64, f64) {
    let t146 = t40 <= zeta_threshold;
    let t2439 = piecewise3(t146, 0.0_f64, 4.0_f64 / 9.0_f64 * t2433 * t2244 + 4.0_f64 / 3.0_f64 * t73 * t2250);
    let t2440 = 1.0_f64 / t197;
    (t2439, t2440)
}
