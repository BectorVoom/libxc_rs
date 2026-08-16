//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1678/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1678(t1352: f64, t19956: f64, t5248: f64, t5250: f64, t5249: f64, t5287: f64, t19871: f64, t120: f64, t6330: f64, t12419: f64, t6347: f64, t3805: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19962 = t5248 * t19956 * t1352;
    let t19966 = t5248 * t19956 * t5250;
    let t19972 = t5248 * t5249 * t5287;
    let t19976 = t5248 * t19871 * t1352;
    let t19979 = t120 * t6330;
    let t19981 = t12419 * t19979 * t1352;
    let t19984 = t120 * t6347;
    let t19986 = t3805 * t19984 * t1352;
    (t19962, t19966, t19972, t19976, t19981, t19986)
}
