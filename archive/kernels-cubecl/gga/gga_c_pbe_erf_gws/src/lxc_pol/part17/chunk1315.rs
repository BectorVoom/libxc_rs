//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1315/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1315<F: Float>(t14046: F, t14522: F, t3261: F, t51214: F, t51306: F, t9506: F, t4026: F, t863: F, t885: F, t338: F, t8828: F, t14011: F, t9581: F) -> (F, F, F, F, F) {
    let t54236 = t14046 * t14522;
    let t54237 = F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t54236;
    let t54238 = t51214 * t3261;
    let t54239 = F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t54238;
    let t54241 = t51306 * t9506;
    let t54244 = t863 * t4026 * t885;
    let t54246 = t54244 * t338 * t8828;
    let t54248 = t14011 * t9581;
    (t54237, t54239, t54241, t54246, t54248)
}
