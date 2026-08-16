//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1409/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1409(t3850: f64, t562: f64, t1339: f64, t836: f64, t1336: f64, t3809: f64, t12248: f64, t236: f64, t3777: f64, t3798: f64, t1354: f64, t12189: f64, t1329: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12272 = t562 * t3850;
    let t12282 = t1339 * t836;
    let t12283 = t1336 * t12282;
    let t12284 = t12283 * t3809;
    let t12289 = t12248 * t236;
    let t12300 = t3777 * t3798;
    let t12301 = t12300 * t1354;
    let t12308 = t12189 * t1329;
    (t12272, t12283, t12284, t12289, t12300, t12301, t12308)
}
