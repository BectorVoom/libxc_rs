//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 631/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk631(t109: f64, t577: f64, t671: f64, t7014: f64, t7017: f64, t7019: f64, t7415: f64, t7423: f64, t33: f64, t68: f64, t69: f64, t79: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t110 = 1.0_f64 < t109;
    let t7426 = 0.45e1_f64 * t7415 * t577 + 0.135e2_f64 * t7423 * t671 + t7014 + t7017 + t7019;
    let t8301 = t33 * t33;
    let t8306 = 1.0_f64 / t69 / t68;
    let t8307 = t79 * t79;
    let t8308 = t8306 * t8307;
    let t8326 = piecewise3(t110, 0.0_f64, 0.0_f64);
    (t7426, t8301, t8306, t8307, t8308, t8326)
}
