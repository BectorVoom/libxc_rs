//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1139/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1139<F: Float>(t2797: F, t8075: F, t10825: F, t23951: F, t24459: F, t24660: F, t24989: F, t25348: F, t25472: F, t25591: F, t25865: F, t25877: F, t25878: F, t25883: F, t25888: F, t25902: F, t25905: F, t25908: F, t25913: F, t2633: F, t2672: F, t2721: F, t2812: F, t2813: F, t297: F, t313: F, t323: F, t3884: F, t914: F, t930: F, t953: F) -> (F,) {
    let t25915 = t2797 * t8075;
    let t25917 = -0.38640729216933594422e6 * t24989 * t313 * t25865 * t297 + 0.10508593825783314861e7 * t25877 * t323 * t25878 * t2672 - 0.75061384469880820436e5 * t25883 * t323 * t25878 * t297 + 0.44430618325890501511e2 * t25888 + 0.47123383072914168269e1 * t2721 * t10825 * t25348 - 0.11721316454988582616e4 * t3884 * t25472 * t24459 + 0.15454509315180013964e0 * t930 * t914 * t2633 * t23951 + 0.82101888746963877062e-1 * t953 * t24660 + 0.1343485452223045261e-1 * t25902 + 0.3118959061058811624e2 * t25905 + 0.12388982497197637389e3 * t25908 - 0.4678438591588217436e2 * t2812 * t2813 * t25591 - 0.30909018630360027928e0 * t25913 + 0.61818037260720055856e0 * t25915;
    (t25917,)
}
