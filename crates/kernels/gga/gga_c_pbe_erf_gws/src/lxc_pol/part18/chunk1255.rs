//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1255/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1255<F: Float>(t54166: F, t14028: F, t3299: F, t2127: F, t3258: F, t850: F, t14046: F, t14522: F, t3261: F, t51214: F, t4026: F, t863: F, t885: F) -> (F, F, F, F, F, F) {
    let t54167 = F::new(7.0) / F::new(144.0) * t54166;
    let t54198 = t14028 * t3299;
    let t54199 = F::new(7.0) / F::new(576.0) * t54198;
    let t54230 = t850 * t3258 * t2127;
    let t54236 = t14046 * t14522;
    let t54237 = F::new(7.0) / F::new(144.0) * t54236;
    let t54238 = t51214 * t3261;
    let t54239 = F::new(7.0) / F::new(288.0) * t54238;
    let t54244 = t863 * t4026 * t885;
    (t54167, t54199, t54230, t54237, t54239, t54244)
}
