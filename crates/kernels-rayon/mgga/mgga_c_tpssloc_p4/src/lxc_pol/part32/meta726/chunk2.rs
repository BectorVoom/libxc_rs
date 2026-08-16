//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2343/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2343(t28: f64, t265: f64, t504: f64, t100624: f64, t104708: f64, t100805: f64, t1409: f64, t16558: f64, t2161: f64, t27850: f64, t29840: f64, t3966: f64, t52: f64, t5398: f64, t607: f64, t7402: f64, t8097: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> f64 {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t104709 = piecewise3(t505, t104708, t100624);
    let t104721 = piecewise3(t401, t100805, t104709 * t52 / 2.0_f64 - t29840 * t607 / 2.0_f64 - t27850 * t1409 - t8097 * t3966 - t7402 * t5398 / 2.0_f64 - t2161 * t16558 / 2.0_f64);
    t104721
}
