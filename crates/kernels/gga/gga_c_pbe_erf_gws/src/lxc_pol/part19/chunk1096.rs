//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1096/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1096<F: Float>(t3261: F, t51214: F, t4026: F, t863: F, t885: F, t828: F, t3287: F, t51255: F, t3142: F, t51382: F, t1125: F, t51292: F, t14024: F, t3120: F, t21296: F, t367: F, t899: F) -> (F, F, F, F, F, F, F, F) {
    let t54238 = t51214 * t3261;
    let t54244 = t863 * t4026 * t885;
    let t54253 = t4026 * t828;
    let t54257 = t51255 * t3287;
    let t54259 = t51382 * t3142;
    let t54267 = t1125 * t51292;
    let t54271 = t3120 * t14024;
    let t54279 = t899 * t21296 * t367;
    (t54238, t54244, t54253, t54257, t54259, t54267, t54271, t54279)
}
