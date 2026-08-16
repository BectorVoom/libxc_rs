//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 647/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk647(t8865: f64, t8963: f64, t752: f64, t2594: f64, t7293: f64, t5218: f64, t747: f64, t8939: f64, t746: f64, t1948: f64, t196: f64, t8616: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8964 = t8865 + t8963;
    let t8965 = t8964 * t752;
    let t8967 = 2.0_f64 * t7293 * t2594;
    let t8968 = t2594 * t2594;
    let t8970 = 2.0_f64 * t5218 * t8968;
    let t8971 = t747 * t8939;
    let t8972 = t746 * t8971;
    let t8973 = t1948 * t8972;
    let t8975 = t8616 * t196;
    (t8964, t8965, t8967, t8968, t8970, t8971, t8972, t8973, t8975)
}
