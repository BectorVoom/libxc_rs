//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1011/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1011(t3070: f64, t851: f64, t2197: f64, t1185: f64, t2234: f64, t2198: f64, t3073: f64, t6142: f64, t2242: f64, t3069: f64, t2240: f64, t1184: f64, t6201: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8189 = t3070 * t851;
    let t8191 = 4.0_f64 * t2197 * t8189;
    let t8192 = t1185 * t2234;
    let t8194 = 2.0_f64 * t2197 * t8192;
    let t8195 = t3073 * t2198;
    let t8197 = 0.96491876992155210402e2_f64 * t6142 * t8195;
    let t8198 = t3069 * t2242;
    let t8199 = t8198 * t851;
    let t8201 = 0.32163958997385070134e2_f64 * t2240 * t8199;
    let t8202 = t3073 * t2234;
    let t8204 = 0.16081979498692535067e2_f64 * t2240 * t8202;
    let t8205 = t1184 * t6201;
    (t8189, t8191, t8192, t8194, t8195, t8197, t8198, t8199, t8201, t8202, t8204, t8205)
}
