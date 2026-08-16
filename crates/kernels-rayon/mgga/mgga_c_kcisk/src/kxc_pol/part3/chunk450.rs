//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 450/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk450(t3532: f64, t459: f64, t3278: f64, t3530: f64, t1337: f64, t306: f64, t1163: f64, t1175: f64, t1422: f64, t425: f64, t1364: f64, t1390: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3533 = t459 * t3532;
    let t3535 = t3530 * t3533 * t3278;
    let t3538 = t1337 * t306;
    let t3539 = t3538 * t459;
    let t3540 = t1163 * t1175;
    let t3541 = t3539 * t3540;
    let t3544 = t1422 * t425;
    let t3545 = t1163 * t1364;
    let t3546 = t3544 * t3545;
    let t3549 = t459 * t1390;
    (t3533, t3535, t3539, t3541, t3544, t3546, t3549)
}
