//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1859/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1859(t25: f64, t265: f64, t394: f64, t13493: f64, t14666: f64, t14673: f64, t1074: f64, t12606: f64, t13503: f64, t13504: f64, t13506: f64, t1408: f64, t1409: f64, t1534: f64, t1642: f64, t2249: f64, t2250: f64, t2756: f64, t3220: f64, t396: f64, t3966: f64, t40: f64, t4324: f64, t4705: f64, t606: f64, t607: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64) {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t14675 = piecewise3(t395, t14666 + t14673, t13493);
    let t14687 = piecewise3(t115, t13493 * t25 / 2.0_f64 + t4324 * t606 + t1534 * t2249 / 2.0_f64 + t2756 * t1408 / 2.0_f64 + t13503 + t13504 - t13506, t14675 * t40 / 2.0_f64 + t4705 * t607 + t1642 * t2250 / 2.0_f64 + t3220 * t1409 / 2.0_f64 + t1074 * t3966 + t396 * t12606 / 2.0_f64);
    (t14675, t14687)
}
