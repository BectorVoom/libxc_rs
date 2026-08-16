//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1124/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1124(t5221: f64, t8939: f64, t16388: f64, t3403: f64, t3407: f64, t5264: f64, t17043: f64, t8978: f64, t6892: f64, t8921: f64, t5257: f64, t8964: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t24087 = t5221 * t8939;
    let t24089 = t16388 * t3403;
    let t24096 = t5264 * t3407;
    let t24135 = t17043 * t8978;
    let t24137 = t6892 * t8921;
    let t24155 = t5257 * t8964;
    (t24087, t24089, t24096, t24135, t24137, t24155)
}
