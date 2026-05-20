//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2928/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2928<F: Float>(t53251: F, t53272: F, t11223: F, t1678: F, t16163: F, t3169: F, t1041: F, t11262: F, t4868: F, t1058: F, t15859: F, t3201: F, t4794: F) -> (F, F, F, F, F, F) {
    let t53273 = t53251 + t53272;
    let t53281 = t11223 * t1678;
    let t53290 = t3169 * t16163;
    let t53293 = t1041 * t11262 * t4868;
    let t53294 = F::cast_from(0.14291339372689912324e-3_f64) * t53293;
    let t53298 = t15859 * t1058;
    let t53300 = t4794 * t3201;
    (t53273, t53281, t53290, t53294, t53298, t53300)
}
