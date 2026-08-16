//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 1085/1228 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk1085(t14164: f64, t17686: f64, t4582: f64, t17691: f64, t4583: f64, t1023: f64, t17670: f64, t4594: f64, t17167: f64, t977: f64, t17171: f64, t17157: f64, t2979: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t17971 = t14164 * t17686;
    let t17972 = t4582 * t17971;
    let t17975 = t4583 * t17691;
    let t17976 = t4582 * t17975;
    let t17979 = t17670 * t1023;
    let t17980 = t4582 * t17979;
    let t17983 = t17670 * t4594;
    let t17984 = t4582 * t17983;
    let t17988 = t977 * t17167;
    let t17991 = t977 * t17171;
    let t17994 = t2979 * t17157;
    (t17972, t17976, t17980, t17984, t17988, t17991, t17994)
}
