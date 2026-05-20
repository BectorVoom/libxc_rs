//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3011/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3011<F: Float>(t12047: F, t53552: F, t15810: F, t3127: F, t3172: F, t1063: F, t11262: F, t4802: F, t4807: F, t11859: F, t11922: F, t15894: F) -> (F, F, F, F, F) {
    let t55046 = t12047 * t53552;
    let t55058 = t3127 * t3172 * t15810;
    let t55061 = t1063 * t11262 * t4802;
    let t55062 = F::cast_from(0.19055119163586549765e-3_f64) * t55061;
    let t55064 = t1063 * t11262 * t4807;
    let t55065 = F::cast_from(0.15879265969655458138e-3_f64) * t55064;
    let t55067 = t11859 * t11922 * t15894;
    (t55046, t55058, t55062, t55065, t55067)
}
