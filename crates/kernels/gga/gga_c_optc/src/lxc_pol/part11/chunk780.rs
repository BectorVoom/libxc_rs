//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 780/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk780<F: Float>(t5355: F, t9142: F, t3244: F, t1178: F, t15326: F, t1160: F, t284: F, t5275: F) -> (F, F, F, F) {
    let t15873 = t9142 * t5355;
    let t15874 = t3244 * t15873;
    let t15889 = t1178 * t15326;
    let t15911 = t1160 * t5275 * t284;
    (t15873, t15874, t15889, t15911)
}
