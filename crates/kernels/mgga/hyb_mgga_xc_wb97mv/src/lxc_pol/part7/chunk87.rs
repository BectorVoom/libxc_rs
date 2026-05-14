//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 87/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk87<F: Float>(t109: F, t209: F, t116: F, t208: F, t193: F, t198: F, t202: F, t94: F) -> (F, F, F, F) {
    let t210 = t109 * t209;
    let t211 = t116 + t208;
    let t212 = 1.0 / t211;
    let t214 = t94 + 0.4e-2 * t193 * t198 * t202 + t210 * t212;
    (t210, t211, t212, t214)
}
