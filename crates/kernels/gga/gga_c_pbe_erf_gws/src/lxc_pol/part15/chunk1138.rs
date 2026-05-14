//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1138/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1138<F: Float>(t338: F, t54090: F, t8886: F, t1125: F, t51221: F, t14011: F, t9393: F, t14498: F, t9401: F, t3179: F, t51291: F, t854: F, t51244: F, t54075: F, t54077: F, t54080: F, t54082: F, t54085: F, t54088: F) -> (F,) {
    let t54092 = t54090 * t338 * t8886;
    let t54094 = t1125 * t51221;
    let t54096 = t14011 * t9393;
    let t54098 = t14498 * t9401;
    let t54101 = t51291 * t3179;
    let t54102 = t854 * t54101;
    let t54103 = 7.0 / 72.0 * t54102;
    let t54104 = -t54075 / 48.0 + t54077 / 768.0 - t54080 / 48.0 + t54082 / 48.0 - t54085 / 48.0 + t54088 - t54092 / 12.0 + 35.0 / 432.0 * t54094 - t54096 / 768.0 + t54098 / 128.0 - 7.0 / 288.0 * t51244 + t54103;
    (t54104,)
}
