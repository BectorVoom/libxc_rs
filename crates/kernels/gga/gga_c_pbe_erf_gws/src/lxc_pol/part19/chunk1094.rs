//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1094/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1094<F: Float>(t3065: F, t3167: F, t3253: F, t51255: F, t14099: F, t863: F, t885: F, t1125: F, t51221: F, t3179: F, t51291: F, t854: F, t3228: F, t51465: F, t3224: F, t1114: F, t51266: F) -> (F, F, F, F, F, F, F, F, F) {
    let t54084 = t3065 * t3167;
    let t54087 = t51255 * t3253;
    let t54090 = t863 * t14099 * t885;
    let t54094 = t1125 * t51221;
    let t54101 = t51291 * t3179;
    let t54102 = t854 * t54101;
    let t54113 = t51465 * t3228;
    let t54117 = t51465 * t3224;
    let t54119 = t1114 * t51266;
    (t54084, t54087, t54090, t54094, t54101, t54102, t54113, t54117, t54119)
}
