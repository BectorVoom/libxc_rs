//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1040/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1040(t3850: f64, t562: f64, t1352: f64, t12240: f64, t3806: f64, t5248: f64, t1339: f64, t836: f64, t1336: f64, t3809: f64, t3777: f64, t3789: f64) -> (f64, f64, f64, f64, f64) {
    let t12272 = t562 * t3850;
    let t12273 = t12272 * t1352;
    let t12279 = t5248 * t3806 * t12240;
    let t12282 = t1339 * t836;
    let t12283 = t1336 * t12282;
    let t12284 = t12283 * t3809;
    let t12286 = t3777 * t3789;
    (t12272, t12273, t12279, t12284, t12286)
}
