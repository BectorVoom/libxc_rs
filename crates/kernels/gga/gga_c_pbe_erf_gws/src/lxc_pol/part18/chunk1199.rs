//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1199/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1199<F: Float>(t11348: F, t4002: F, t13808: F, t15151: F, t12182: F, t13792: F, t1115: F, t11342: F, t13772: F, t3921: F, t51967: F, t54617: F, t54711: F, t55892: F, t57626: F, t57635: F, t57639: F, t57641: F, t57643: F, t57648: F) -> (F,) {
    let t57650 = t11348 * t4002;
    let t57652 = t13808 * t15151;
    let t57654 = t13792 * t12182;
    let t57656 = -t1115 * t54711 / 48.0 - t57626 / 768.0 - t11342 * t4002 / 96.0 - t3921 * t13772 / 96.0 - t57635 / 1536.0 + t54617 - 35.0 / 432.0 * t51967 - t55892 - t57639 / 96.0 + 7.0 / 1152.0 * t57641 + 7.0 / 288.0 * t57643 - t57648 / 768.0 + 7.0 / 288.0 * t57650 + 7.0 / 2304.0 * t57652 + t57654 / 24.0;
    (t57656,)
}
