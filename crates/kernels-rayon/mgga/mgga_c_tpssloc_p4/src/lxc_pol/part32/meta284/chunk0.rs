//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1286/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1286(t28: f64, t265: f64, t504: f64, t1256: f64, t1763: f64, t193: f64, t336: f64, t4700: f64, t7398: f64, t7642: f64, t8090: f64, t1409: f64, t2161: f64, t52: f64, t7663: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> (f64, f64) {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t8097 = piecewise3(t505, t1256 * t193 * t336 * t8090 - t1763 * t4700 * t7398, t7642);
    let t8102 = piecewise3(t401, t7663, -t2161 * t1409 / 2.0_f64 + t8097 * t52 / 2.0_f64);
    (t8097, t8102)
}
