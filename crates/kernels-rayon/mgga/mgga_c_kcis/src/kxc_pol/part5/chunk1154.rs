//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1154/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1154(t6436: f64, t733: f64, t6439: f64, t1056: f64, t18681: f64, t1064: f64, t1079: f64, t6470: f64, t743: f64, t6461: f64, t738: f64, t6464: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t19423 = t733 * t6436;
    let t19425 = t733 * t6439;
    let t19427 = t1056 * t18681;
    let t19430 = t1064 * t18681;
    let t19433 = t1079 * t18681;
    let t19436 = t743 * t6470;
    let t19438 = t738 * t6461;
    let t19440 = t738 * t6464;
    (t19423, t19425, t19427, t19430, t19433, t19436, t19438, t19440)
}
