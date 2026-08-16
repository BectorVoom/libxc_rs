//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 512/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk512(t2325: f64, t1237: f64, t1356: f64, t1358: f64, t1360: f64, t1378: f64, t1387: f64, t1389: f64, t1413: f64, t1418: f64, t1783: f64, t2065: f64, t2068: f64, t2265: f64, t2270: f64, t2272: f64, t2322: f64, t372: f64, t881: f64) -> f64 {
    let t2326 = 6.0_f64 * t2325;
    let t2327 = -0.4726e1_f64 * t2272 - 0.2363e1_f64 * t881 * t2065 - 0.4726e1_f64 * t881 * t2068 - t1237 + t1356 - t1358 - t1360 - t1378 - t2265 + t1387 - t2270 + t1389 + t1413 + t372 * t1783 + t2322 - t2326 - t1418;
    t2327
}
