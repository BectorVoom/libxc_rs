//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1261/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1261<F: Float>(t14928: F, t840: F, t53873: F, t15018: F, t53896: F, t54014: F, t54052: F, t54072: F, t54087: F, t54102: F, t54113: F, t54117: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t55385 = F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t840 * t14928;
    let t55403 = F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t53873;
    let t55420 = F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t840 * t15018;
    let t55421 = F::cast_from(7.0_f64) / F::cast_from(36.0_f64) * t53896;
    let t55432 = F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t54014;
    let t55452 = F::cast_from(7.0_f64) / F::cast_from(96.0_f64) * t54052;
    let t55460 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t54072;
    let t55467 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t54087;
    let t55473 = F::cast_from(7.0_f64) / F::cast_from(36.0_f64) * t54102;
    let t55480 = F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t54113;
    let t55482 = F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t54117;
    (t55385, t55403, t55420, t55421, t55432, t55452, t55460, t55467, t55473, t55480, t55482)
}
