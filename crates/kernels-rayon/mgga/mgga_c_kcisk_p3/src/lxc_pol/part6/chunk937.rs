//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 937/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk937(t29593: f64, t735: f64, t734: f64, t28303: f64, t7311: f64, t5321: f64, t2586: f64, t8971: f64, t1948: f64, t2572: f64, t9016: f64, t28963: f64, t719: f64) -> (f64, f64, f64, f64, f64) {
    let t29594 = t735 * t29593;
    let t29595 = t734 * t29594;
    let t29597 = t7311 * t28303;
    let t29598 = t5321 * t29597;
    let t29600 = t2586 * t8971;
    let t29601 = t1948 * t29600;
    let t29603 = t9016 * t2572;
    let t29605 = t719 * t28963;
    (t29595, t29598, t29601, t29603, t29605)
}
