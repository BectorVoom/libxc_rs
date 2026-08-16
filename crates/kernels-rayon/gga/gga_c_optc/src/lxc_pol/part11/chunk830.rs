//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 830/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk830(t4492: f64, t4509: f64, t4501: f64, t4512: f64, t1157: f64, t5421: f64, t4380: f64, t4444: f64, t1179: f64, t15335: f64, t2367: f64, t5102: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16035 = t4492 * t4509;
    let t16037 = t4501 * t4512;
    let t16055 = t5421 * t1157;
    let t16071 = t4444 * t4380;
    let t16073 = t1179 * t15335;
    let t16094 = t2367 * t5102;
    (t16035, t16037, t16055, t16071, t16073, t16094)
}
