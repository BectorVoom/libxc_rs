//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 746/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk746(t4039: f64, t7178: f64, t2722: f64, t1006: f64, t2317: f64, t1009: f64, t2248: f64, t2253: f64, t2247: f64, t2554: f64, t280: f64, t303: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7179 = t4039 * t7178;
    let t7180 = t2722 * t7179;
    let t7182 = t1006 * t2317;
    let t7183 = t7182 * t1009;
    let t7186 = t2248 * t2253;
    let t7188 = t2554 * t2247;
    let t7192 = 1.0_f64 / t280 / t303;
    (t7179, t7180, t7182, t7183, t7186, t7188, t7192)
}
