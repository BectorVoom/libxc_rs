//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 755/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk755(t15422: f64, t933: f64, t116: f64, t12769: f64, t982: f64, t979: f64, t119: f64, t3127: f64, t140: f64, t191: f64, t1002: f64, t3174: f64) -> (f64, f64, f64, f64) {
    let t15423 = t15422 * t933;
    let t15426 = t116 * t12769;
    let t15427 = t982 * t15426;
    let t15428 = t979 * t15427;
    let t15430 = t119 * t3127;
    let t15432 = t140 * t15430 * t191;
    let t15434 = t1002 * t3174;
    (t15423, t15428, t15432, t15434)
}
