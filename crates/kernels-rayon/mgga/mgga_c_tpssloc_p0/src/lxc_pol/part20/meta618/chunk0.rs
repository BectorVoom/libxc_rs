//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2229/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2229(t40722: f64, t40726: f64, t12858: f64, t2528: f64, t2371: f64, t40729: f64, t40733: f64, t2745: f64, t776: f64, t4205: f64, t9909: f64, t2553: f64, t868: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t46228 = 0.17090684152272775383e-2_f64 * t40722;
    let t46232 = 24.0_f64 * t40726;
    let t46234 = t12858 * t2528;
    let t46235 = 0.51947577317044391276e2_f64 * t46234;
    let t46236 = t12858 * t2371;
    let t46237 = 0.35089341735807877242e1_f64 * t46236;
    let t46238 = 36.0_f64 * t40729;
    let t46239 = 0.10526802520742363173e2_f64 * t40733;
    let t46240 = t2745 * t776;
    let t46244 = t4205 * t9909;
    let t46245 = 12.0_f64 * t46244;
    let t46252 = t2553 * t868;
    (t46228, t46232, t46235, t46237, t46238, t46239, t46240, t46245, t46252)
}
