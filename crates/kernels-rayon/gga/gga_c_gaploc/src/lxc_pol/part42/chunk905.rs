//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 905/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk905(t44295: f64, t4820: f64, t6824: f64, t13387: f64, t4379: f64, t11218: f64, t1429: f64, t2365: f64, t2366: f64, t11430: f64, t2389: f64, t44294: f64, t475: f64) -> (f64, f64, f64, f64, f64) {
    let t46244 = 0.79445533226334281487e-1_f64 * t6824 * t4820 * t44295;
    let t46245 = t4379 * t13387;
    let t46246 = 0.14896037479937677779e-1_f64 * t46245;
    let t46249 = t1429 * t2365 * t2366 * t11218;
    let t46250 = 0.14896037479937677779e-1_f64 * t46249;
    let t46251 = t11430 * t2389;
    let t46252 = 0.29792074959875355558e-1_f64 * t46251;
    let t46254 = t44294 * t475;
    (t46244, t46246, t46250, t46252, t46254)
}
