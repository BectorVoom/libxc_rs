//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 335/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk335(t1234: f64, t471: f64, t97: f64, t292: f64, t800: f64, t297: f64, t806: f64, t1218: f64, t298: f64, t307: f64, t810: f64, t308: f64, rho0: f64, tau0: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1236 = t97 * t471 * t1234;
    let t1237 = 3.0_f64 * t1236;
    let t1242 = 1.0_f64 / t292 / t800 / rho0;
    let t1243 = tau0 * t1242;
    let t1248 = 1.0_f64 / t297;
    let t1249 = t806 * t806;
    let t1250 = t1248 * t1249;
    let t1253 = t298 * t1218;
    let t1256 = 1.0_f64 / t307;
    let t1257 = t810 * t810;
    let t1258 = t1256 * t1257;
    let t1261 = -t1218;
    let t1262 = t308 * t1261;
    (t1237, t1243, t1248, t1249, t1250, t1253, t1256, t1257, t1258, t1261, t1262)
}
