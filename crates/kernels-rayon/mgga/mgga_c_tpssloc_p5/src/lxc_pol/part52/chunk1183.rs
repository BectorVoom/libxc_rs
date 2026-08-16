//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1183/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1183(t25: f64, t265: f64, t394: f64, t31683: f64, t8308: f64, t1862: f64, t79: f64, t641: f64, t8513: f64, t6534: f64, t88: f64, t30952: f64, t30776: f64, t40: f64, t607: f64, t8678: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t31684 = t8308 * t31683;
    let t31691 = t79 * t1862;
    let t31693 = t8513 * t31691 * t641;
    let t31717 = t88 * t6534;
    let t31823 = piecewise3(t395, 0.0_f64, t30952);
    let t31828 = piecewise3(t115, t30776, t31823 * t40 / 2.0_f64 + t8678 * t607 / 2.0_f64);
    (t31684, t31691, t31693, t31717, t31823, t31828)
}
