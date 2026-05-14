//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1266/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1266<F: Float>(t31884: F, t31998: F, t31999: F, t1053: F, t3186: F, t31860: F, t3181: F, t32581: F, t43151: F, t37229: F, t9340: F, t10335: F, t2685: F, t10337: F, t10349: F, t31883: F) -> (F, F, F, F, F, F, F, F, F) {
    let t109144 = 6.0 * t31884;
    let t109148 = 3.0 * t31998;
    let t109149 = 3.0 * t31999;
    let t109152 = 6.0 * t3186 * t31860 * t1053;
    let t109154 = 3.0 * t3181 * t31860;
    let t109160 = 18.0 * t43151 * t32581;
    let t109162 = 6.0 * t37229 * t9340;
    let t109163 = t2685 * t10335;
    let t109165 = 6.0 * t109163 * t10337;
    let t110815 = 6.0 * t31883 * t10349;
    (t109144, t109148, t109149, t109152, t109154, t109160, t109162, t109165, t110815)
}
