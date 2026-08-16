//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1074/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1074(t127: f64, t495: f64, t5136: f64, t4803: f64, t546: f64, t1670: f64, t5322: f64, t5119: f64, t545: f64, t83: f64, t16129: f64, t81: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16929 = t495 * t5136 * t127;
    let t16931 = t4803 * t546;
    let t16935 = t1670 * t5322;
    let t16938 = t83 * t5119 * t545;
    let t16940 = t16129 * t127;
    let t16942 = t81 * t81;
    (t16929, t16931, t16935, t16938, t16940, t16942)
}
