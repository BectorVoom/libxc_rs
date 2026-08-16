//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1254/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1254(t25355: f64, t3907: f64, t8003: f64, t2797: f64, t8019: f64, t8075: f64, t10825: f64, t23951: f64, t24459: f64, t24660: f64, t24989: f64, t25348: f64, t25472: f64, t25591: f64, t25865: f64, t25877: f64, t25878: f64, t25883: f64, t25888: f64, t25902: f64, t25905: f64, t2633: f64, t2672: f64, t2721: f64, t2812: f64, t2813: f64, t297: f64, t313: f64, t323: f64, t3884: f64, t914: f64, t930: f64, t953: f64) -> f64 {
    let t25908 = t3907 * t25355 * t8003;
    let t25913 = t2797 * t8019;
    let t25915 = t2797 * t8075;
    let t25917 = -0.38640729216933594422e6_f64 * t24989 * t313 * t25865 * t297 + 0.10508593825783314861e7_f64 * t25877 * t323 * t25878 * t2672 - 0.75061384469880820436e5_f64 * t25883 * t323 * t25878 * t297 + 0.44430618325890501511e2_f64 * t25888 + 0.47123383072914168269e1_f64 * t2721 * t10825 * t25348 - 0.11721316454988582616e4_f64 * t3884 * t25472 * t24459 + 0.15454509315180013964e0_f64 * t930 * t914 * t2633 * t23951 + 0.82101888746963877062e-1_f64 * t953 * t24660 + 0.1343485452223045261e-1_f64 * t25902 + 0.3118959061058811624e2_f64 * t25905 + 0.12388982497197637389e3_f64 * t25908 - 0.4678438591588217436e2_f64 * t2812 * t2813 * t25591 - 0.30909018630360027928e0_f64 * t25913 + 0.61818037260720055856e0_f64 * t25915;
    t25917
}
