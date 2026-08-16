//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 839/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk839(t25: f64, t265: f64, t394: f64, t1458: f64, t2165: f64, t7642: f64, t1409: f64, t2116: f64, t40: f64, t7552: f64, t1419: f64, t337: f64, t1887: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64) {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t7989 = t2165 * t1458;
    let t7992 = piecewise3(t395, 0.0_f64, t7642);
    let t7997 = piecewise3(t115, t7552, t2116 * t1409 / 2.0_f64 + t7992 * t40 / 2.0_f64);
    let t7998 = t1419 * t337;
    let t7999 = t7998 * t1887;
    (t7989, t7992, t7997, t7998, t7999)
}
