//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2336/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2336(t28: f64, t265: f64, t504: f64, t89823: f64, t95952: f64, t12606: f64, t1409: f64, t2161: f64, t2250: f64, t24916: f64, t27850: f64, t3966: f64, t52: f64, t607: f64, t7402: f64, t8097: f64, t90003: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> f64 {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t95953 = piecewise3(t505, t95952, t89823);
    let t95965 = piecewise3(t401, t90003, t95953 * t52 / 2.0_f64 - t27850 * t607 - t8097 * t2250 / 2.0_f64 - t24916 * t1409 / 2.0_f64 - t7402 * t3966 - t2161 * t12606 / 2.0_f64);
    t95965
}
