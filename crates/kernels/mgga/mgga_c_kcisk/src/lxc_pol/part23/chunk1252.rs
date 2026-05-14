//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1252/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1252<F: Float>(t3924: F, t3959: F, t2326: F, t4350: F, t3179: F, t3185: F, t1009: F, t15451: F, t3923: F, t3729: F, t3929: F, t1216: F, t13436: F, t13435: F, t338: F, t411: F) -> (F, F, F, F, F, F, F, F) {
    let t35869 = t3959 * t3924;
    let t36521 = t4350 * t2326;
    let t37229 = t3179 * t3185;
    let t37234 = t15451 * t1009;
    let t39809 = t3923 * t3923;
    let t39810 = 1.0 / t39809;
    let t39814 = t3729 * t3929;
    let t41006 = t1216 * t13436;
    let t41167 = t338 / t13435 / t411;
    (t35869, t36521, t37229, t37234, t39810, t39814, t41006, t41167)
}
