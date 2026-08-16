//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 679/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk679(t3147: f64, t900: f64, t1217: f64, t2328: f64, t1208: f64, t2295: f64, t891: f64, t898: f64, t3135: f64, t881: f64, t890: f64, t2317: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3149 = 0.5848223622634646207e0_f64 * t3147 * t900;
    let t3151 = 0.5848223622634646207e0_f64 * t2328 * t1217;
    let t3152 = t2295 * t1208;
    let t3153 = t3152 * t891;
    let t3155 = 0.11696447245269292414e1_f64 * t898 * t3153;
    let t3157 = t881 * t3135 * t890;
    let t3159 = 0.5848223622634646207e0_f64 * t898 * t3157;
    let t3160 = t2317 * t1208;
    (t3149, t3151, t3152, t3153, t3155, t3157, t3159, t3160)
}
