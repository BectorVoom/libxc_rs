//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 868/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk868(t11849: f64, t2628: f64, t43646: f64, t43652: f64, t43657: f64, t43660: f64, t43679: f64, t43681: f64, t11848: f64, t2021: f64, t7372: f64, t11576: f64, t123: f64, t883: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t45441 = t11849 * t2628;
    let t45442 = 0.29792074959875355558e-1_f64 * t45441;
    let t45451 = 0.17875244975925213335e0_f64 * t43646;
    let t45453 = 0.30674340763136599741e1_f64 * t43652;
    let t45454 = 0.20449560508757733161e1_f64 * t43657;
    let t45457 = 0.34082600847929555269e0_f64 * t43660;
    let t45458 = 0.59584149919750711116e-1_f64 * t43679;
    let t45459 = 0.71500979903700853339e0_f64 * t43681;
    let t45463 = t2021 * t11848 * t7372;
    let t45464 = 0.14896037479937677779e-1_f64 * t45463;
    let t45466 = t11576 * t123 * t883;
    (t45442, t45451, t45453, t45454, t45457, t45458, t45459, t45464, t45466)
}
