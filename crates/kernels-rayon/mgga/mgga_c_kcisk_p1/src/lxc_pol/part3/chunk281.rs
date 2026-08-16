//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 281/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk281(t1320: f64, t1322: f64, t1310: f64, t1294: f64, t1301: f64, t1307: f64, t1309: f64, t1315: f64, t405: f64, t408: f64, t11: f64, t139: f64, t79: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1323 = t1320 * t1322;
    let t1324 = t1310 * t1323;
    let t1327 = 0.5397236614853195164e-1_f64 * t1294 * t405 - 0.14392630972941853771e0_f64 * t1301 * t405 + t1307 + 0.17990788716177317213e-1_f64 * t1309 * t1315 - 0.5397236614853195164e-1_f64 * t1309 * t1324;
    let t1328 = 1.0_f64 / t408;
    let t1329 = t1327 * t1328;
    let t1333 = t139 * t11 * t79;
    (t1323, t1324, t1327, t1328, t1329, t1333)
}
