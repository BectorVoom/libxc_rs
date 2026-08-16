//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 827/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk827(t5355: f64, t9142: f64, t3244: f64, t1178: f64, t15326: f64, t1160: f64, t284: f64, t5275: f64) -> (f64, f64, f64, f64) {
    let t15873 = t9142 * t5355;
    let t15874 = t3244 * t15873;
    let t15889 = t1178 * t15326;
    let t15911 = t1160 * t5275 * t284;
    (t15873, t15874, t15889, t15911)
}
