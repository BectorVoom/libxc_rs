//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1017/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1017(t3176: f64, t68: f64, t3174: f64, t3026: f64, t931: f64, t824: f64, t2888: f64, t2226: f64, t3236: f64, t1238: f64, t2402: f64, t1208: f64, t6230: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8273 = t68 * t3176;
    let t8275 = t3174 * t8273 / 72.0_f64;
    let t8276 = t931 * t3026;
    let t8277 = t8276 * t824;
    let t8278 = t2888 * t8277;
    let t8281 = t3236 * t2226;
    let t8282 = t2888 * t8281;
    let t8285 = t1238 * t2402;
    let t8287 = t6230 * t1208;
    (t8275, t8276, t8277, t8278, t8281, t8282, t8285, t8287)
}
