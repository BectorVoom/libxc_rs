//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1327/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1327(t2995: f64, t3012: f64, t3018: f64, t1057: f64, t2993: f64, t8679: f64, t3021: f64, t8582: f64, t8569: f64, t8577: f64, t1094: f64, t1102: f64, t26229: f64, t2916: f64) -> (f64, f64, f64, f64, f64) {
    let t26476 = 36.0_f64 * t3018 * t2995 * t3012;
    let t26479 = 8.0_f64 * t2993 * t1057 * t8679;
    let t26482 = 0.57894567559743977359e3_f64 * t8582 * t3021 * t3012;
    let t26484 = 0.19298189186581325786e3_f64 * t8577 * t8569;
    let t26488 = 0.35089340384731224426e1_f64 * t1102 * t2916 * t26229 * t1094;
    (t26476, t26479, t26482, t26484, t26488)
}
