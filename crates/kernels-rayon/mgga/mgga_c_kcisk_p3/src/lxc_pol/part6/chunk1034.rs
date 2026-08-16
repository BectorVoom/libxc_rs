//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 1034/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk1034(t1341: f64, t30290: f64, t1340: f64, t3759: f64, t2231: f64, t7710: f64, t3797: f64, t3796: f64, t3482: f64, t2152: f64, t3485: f64, t3484: f64) -> (f64, f64, f64) {
    let t30967 = t1341 * t30290;
    let t30968 = t1340 * t30967;
    let t30969 = t3759 * t30968;
    let t30972 = t7710 * t2231;
    let t30973 = t3797 * t30972;
    let t30974 = t3796 * t30973;
    let t30975 = t3482 * t30974;
    let t30978 = t3485 * t7710 * t2152;
    let t30979 = t3484 * t30978;
    (t30969, t30975, t30979)
}
