//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 939/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk939(t3970: f64, t3974: f64, t3947: f64, t3973: f64, t1309: f64, t25: f64, t3951: f64, t3954: f64, t3943: f64, t12830: f64, t3953: f64, t1312: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13910 = t3970 * t3974;
    let t13912 = t3973 * t3947;
    let t13913 = t1309 * t13912;
    let t13917 = t25 * t3951;
    let t13918 = t13917 * t3954;
    let t13919 = t1309 * t13918;
    let t13923 = t3973 * t3943;
    let t13924 = t1309 * t13923;
    let t13926 = t3953 * t12830;
    let t13927 = t1312 * t13926;
    (t13910, t13913, t13917, t13919, t13924, t13927)
}
