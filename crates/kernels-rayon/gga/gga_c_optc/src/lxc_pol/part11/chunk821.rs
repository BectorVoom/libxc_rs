//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 821/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk821(t241: f64, t5197: f64, t3058: f64, t5218: f64, t5202: f64, t8697: f64, t2586: f64, t5336: f64, t1133: f64, t1108: f64, t5276: f64, t15270: f64, t3109: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t15562 = t241 * t5197;
    let t15571 = t3058 * t5218;
    let t15582 = t8697 * t5202;
    let t15597 = t2586 * t5336;
    let t15598 = t1133 * t15597;
    let t15620 = t5276 * t1108;
    let t15622 = t15270 * t3109;
    (t15562, t15571, t15582, t15597, t15598, t15620, t15622)
}
