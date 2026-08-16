//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 522/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk522(t12: f64, t1646: f64, t2159: f64, t318: f64, t319: f64, t808: f64, t810: f64, t201: f64, t1281: f64, t204: f64, t334: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t84 = t12 <= zeta_threshold;
    let t203 = rho0 <= dens_threshold || t84;
    let t2163 = piecewise3(t84, 0.0_f64, t1646);
    let t2167 = piecewise3(t203, 0.0_f64, t2159 * t319 / 2.0_f64 + t808 * t810 + t318 * t2163 / 2.0_f64);
    let t2168 = t201 * t2167;
    let t2172 = t204 * t1281 * t334;
    (t2163, t2168, t2172)
}
