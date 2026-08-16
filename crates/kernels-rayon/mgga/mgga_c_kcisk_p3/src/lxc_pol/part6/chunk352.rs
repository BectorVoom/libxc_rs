//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 352/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk352(t1354: f64, t2191: f64, t1375: f64, t2075: f64, t1383: f64, t1398: f64, t1349: f64, t1369: f64, t1374: f64, t1382: f64, t1388: f64, t1391: f64, t1397: f64, t158: f64, t165: f64, t173: f64, t2059: f64) -> (f64, f64, f64, f64, f64) {
    let t2192 = t1354 * t2191;
    let t2198 = t1375 * t2075;
    let t2201 = t1383 * t2075;
    let t2206 = t1398 * t2075;
    let t2209 = t1369 + 0.11955719325063177623e-1_f64 * t1349 * t2059 - t1374 - 0.3513e-2_f64 * t158 * t2198 + t1382 + 0.7925e-3_f64 * t165 * t2201 - t1388 - 0.5179538907796306876e-4_f64 * t1391 * t2059 + t1397 + 0.50413125e-5_f64 * t173 * t2206;
    (t2192, t2198, t2201, t2206, t2209)
}
