//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1072/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1072<F: Float>(t1220: F, t17464: F, t2367: F, t15015: F, t4535: F, t15011: F, t4229: F, t1219: F, t176: F, t17612: F, t3116: F, t46945: F, t5324: F, t17941: F, t3103: F, t45304: F) -> (F, F, F, F, F, F) {
    let t53909 = t1220 * t2367 * t17464;
    let t53911 = t4535 * t15015;
    let t53914 = t15011 * t4229;
    let t53918 = t176 * t17612 * t1219;
    let t53950 = t3116 * t46945 * t5324;
    let t53953 = t3103 * t45304 * t17941;
    (t53909, t53911, t53914, t53918, t53950, t53953)
}
