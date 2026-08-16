//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 1074/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk1074(t30852: f64, t4406: f64, t1312: f64, t2059: f64, t21651: f64, t8335: f64, t30153: f64, t4391: f64, t6505: f64, t8398: f64, t6204: f64, t14995: f64) -> (f64, f64, f64, f64, f64) {
    let t31639 = t4406 * t30852;
    let t31640 = t1312 * t31639;
    let t31644 = t21651 * t2059 * t8335;
    let t31645 = t1312 * t31644;
    let t31651 = t4391 * t30153;
    let t31652 = t1312 * t31651;
    let t31655 = t6505 * t8398;
    let t31656 = t6204 * t31655;
    let t31659 = t14995 * t30153;
    (t31640, t31645, t31652, t31656, t31659)
}
