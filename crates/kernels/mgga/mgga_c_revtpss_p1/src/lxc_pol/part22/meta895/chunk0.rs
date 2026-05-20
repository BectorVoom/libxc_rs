//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3086/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3086<F: Float>(t247: F, t42792: F, t4757: F, t4837: F, t15850: F, t3111: F, t3091: F, t43240: F, t4782: F, t41296: F, t42471: F, t11977: F, t4820: F) -> (F, F, F, F, F) {
    let t53431 = t4837 * t247 * t42792 * t4757;
    let t53433 = t15850 * t3111;
    let t53437 = t3091 * t43240 * t4782;
    let t53473 = t42471 * t41296;
    let t53479 = t11977 * t4820;
    (t53431, t53433, t53437, t53473, t53479)
}
