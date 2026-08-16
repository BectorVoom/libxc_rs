//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 806/1444 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk806(t3034: f64, t9223: f64, t3100: f64, t659: f64, t2979: f64, t5856: f64, t1504: f64, t2982: f64, t1875: f64, t9128: f64, t2983: f64, t1027: f64, t1781: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9224 = t3034 * t9223;
    let t9226 = t3100 * t659;
    let t9228 = t5856 * t2979;
    let t9229 = t2982 * t1504;
    let t9230 = t9228 * t9229;
    let t9232 = t1875 * t9128;
    let t9233 = t9232 * t2983;
    let t9235 = t1027 * t1781;
    (t9224, t9226, t9229, t9230, t9233, t9235)
}
