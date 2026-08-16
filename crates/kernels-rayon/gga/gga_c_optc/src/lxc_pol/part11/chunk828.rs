//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 828/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk828(t1129: f64, t5417: f64, t2367: f64, t5403: f64, t1150: f64, t1156: f64, t5398: f64, t3217: f64, t2586: f64, t5388: f64, t1170: f64, t1179: f64, t15597: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t15978 = t5417 * t1129;
    let t15980 = t2367 * t5403;
    let t15981 = t1150 * t15980;
    let t15983 = t1156 * t5398;
    let t15984 = t3217 * t15983;
    let t15986 = t2586 * t5388;
    let t15987 = t1170 * t15986;
    let t15996 = t1179 * t15597;
    (t15978, t15980, t15981, t15983, t15984, t15986, t15987, t15996)
}
