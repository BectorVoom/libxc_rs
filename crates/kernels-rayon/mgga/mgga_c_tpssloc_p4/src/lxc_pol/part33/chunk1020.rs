//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1020/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1020(t25: f64, t265: f64, t394: f64, t21076: f64, t21381: f64, t21701: f64, t1408: f64, t1409: f64, t1534: f64, t1642: f64, t20216: f64, t20217: f64, t396: f64, t40: f64, t5397: f64, t5398: f64, t5669: f64, t5955: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> f64 {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t21703 = piecewise3(t395, t21381 + t21701, t21076);
    let t21713 = piecewise3(t115, t21076 * t25 / 2.0_f64 + 3.0_f64 / 2.0_f64 * t5669 * t1408 + 3.0_f64 / 2.0_f64 * t1534 * t5397 + t265 * t20216 / 2.0_f64, t21703 * t40 / 2.0_f64 + 3.0_f64 / 2.0_f64 * t5955 * t1409 + 3.0_f64 / 2.0_f64 * t1642 * t5398 + t396 * t20217 / 2.0_f64);
    t21713
}
