//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 832/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk832(t12769: f64, t214: f64, t982: f64, t1050: f64, t3174: f64, t3132: f64, t3266: f64, t207: f64, t1035: f64, t1039: f64, t944: f64, t967: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12770 = t214 * t12769;
    let t12771 = t982 * t12770;
    let t12773 = t1050 * t3174;
    let t12774 = t982 * t12773;
    let t12776 = t3132 * t3266;
    let t12778 = t207 * t12769;
    let t12779 = t1035 * t12778;
    let t12781 = t1039 * t3174;
    let t12782 = t1035 * t12781;
    let t12786 = t967 * t944;
    (t12771, t12774, t12776, t12779, t12782, t12786)
}
