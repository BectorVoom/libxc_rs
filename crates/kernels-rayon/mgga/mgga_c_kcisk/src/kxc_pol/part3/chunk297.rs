//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 297/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk297(t1163: f64, t1398: f64, t1056: f64, t1349: f64, t1369: f64, t1374: f64, t1376: f64, t1382: f64, t1384: f64, t1388: f64, t1391: f64, t1397: f64, t158: f64, t165: f64, t173: f64) -> (f64, f64) {
    let t1399 = t1398 * t1163;
    let t1402 = t1369 + 0.11955719325063177623e-1_f64 * t1349 * t1056 - t1374 - 0.3513e-2_f64 * t158 * t1376 + t1382 + 0.7925e-3_f64 * t165 * t1384 - t1388 - 0.5179538907796306876e-4_f64 * t1391 * t1056 + t1397 + 0.50413125e-5_f64 * t173 * t1399;
    (t1399, t1402)
}
