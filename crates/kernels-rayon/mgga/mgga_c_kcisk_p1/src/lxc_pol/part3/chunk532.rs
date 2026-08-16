//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 532/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk532(t1588: f64, t442: f64, t1056: f64, t1591: f64, t1312: f64, t1390: f64, t539: f64, t3278: f64, t1572: f64, t1576: f64, t397: f64, t3979: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4400 = t1588 * t442;
    let t4401 = t1056 * t1591;
    let t4402 = t4400 * t4401;
    let t4403 = t1312 * t4402;
    let t4406 = t539 * t1390;
    let t4407 = t4406 * t3278;
    let t4408 = t1312 * t4407;
    let t4411 = t1572 * t1576;
    let t4416 = t397 * t3979 * t539;
    (t4400, t4402, t4403, t4406, t4407, t4408, t4411, t4416)
}
