//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 851/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk851(t10205: f64, t871: f64, t39624: f64, t39626: f64, t39632: f64, t39646: f64, t39648: f64, t39650: f64, t40353: f64, t9078: f64, t986: f64, t544: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t42114 = t10205 * t871;
    let t42117 = 7.0_f64 / 512.0_f64 * t39624;
    let t42118 = 63.0_f64 / 16384.0_f64 * t39626;
    let t42119 = 63.0_f64 / 1048576.0_f64 * t39632;
    let t42120 = 21.0_f64 / 1048576.0_f64 * t39646;
    let t42121 = 21.0_f64 / 16384.0_f64 * t39648;
    let t42122 = 7.0_f64 / 1536.0_f64 * t39650;
    let t42144 = 0.11502877786176224903e1_f64 * t40353;
    let t42148 = t9078 * t986;
    let t42149 = t544 * t42148;
    (t42114, t42117, t42118, t42119, t42120, t42121, t42122, t42144, t42148, t42149)
}
