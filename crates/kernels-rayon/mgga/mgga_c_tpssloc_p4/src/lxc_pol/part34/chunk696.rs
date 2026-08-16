//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 696/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk696(t25: f64, t265: f64, t394: f64, t1484: f64, t2057: f64, t202: f64, t7844: f64, t1530: f64, t1877: f64, t193: f64, t2522: f64, t7114: f64, t870: f64, t1408: f64, t1409: f64, t2064: f64, t40: f64, t7545: f64, t7809: f64, t7845: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t7856 = t2057 * t1484;
    let t7859 = t202 * t7844;
    let t7864 = -t1530 * t1877 * t7114 + t193 * t7859 * t870 + 3.0_f64 * t2522 * t7856;
    let t7865 = piecewise3(t395, 0.0_f64, t7864);
    let t7870 = piecewise3(t115, 3.0_f64 / 2.0_f64 * t2522 * t7809 + t1877 * t7845 * t25 / 2.0_f64 - t1877 * t7114 * t7545 / 2.0_f64 + t1877 * t2057 * t1408 / 2.0_f64, t2064 * t1409 / 2.0_f64 + t7865 * t40 / 2.0_f64);
    (t7864, t7865, t7870)
}
