//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 276/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk276(t1297: f64, t1303: f64, t1311: f64, t1315: f64, t1323: f64, t1326: f64, t1327: f64, t1330: f64, t255: f64, t261: f64, t262: f64, t331: f64, t831: f64) -> f64 {
    let t1338 = 2.0_f64 * t1297 * t255 - 1.0_f64 * t1303 * t255 + 1.0_f64 * t1311 * t255 + 0.2845018947250181111e-1_f64 * t1315 * t331 - 0.20235332025531322028e-2_f64 * t1323 * t1326 * t1327 * t1330 + 0.52158680699586653702e-1_f64 * t261 * t262 * t831;
    t1338
}
