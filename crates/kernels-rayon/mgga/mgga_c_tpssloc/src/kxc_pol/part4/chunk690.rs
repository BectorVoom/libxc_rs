//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 690/1228 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk690(t25: f64, t265: f64, t394: f64, t4324: f64, t4704: f64, t1074: f64, t1408: f64, t1409: f64, t1534: f64, t1642: f64, t396: f64, t3966: f64, t40: f64, t4332: f64, t606: f64, t607: f64, t873: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64) {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t4705 = piecewise3(t395, t4704, t4324);
    let t4712 = piecewise3(t115, t4324 * t25 / 2.0_f64 + t1534 * t606 / 2.0_f64 + t873 * t1408 / 2.0_f64 + t4332, t1074 * t1409 / 2.0_f64 + t1642 * t607 / 2.0_f64 + t396 * t3966 / 2.0_f64 + t4705 * t40 / 2.0_f64);
    (t4705, t4712)
}
