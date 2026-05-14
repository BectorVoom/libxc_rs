//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1176/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1176<F: Float>(t54531: F, t54535: F, t54537: F, t14182: F, t14185: F, t14188: F, t14882: F, t15036: F, t19895: F, t2408: F, t29751: F, t29775: F, t3066: F, t51928: F, t52560: F, t54519: F, t54523: F, t54529: F, t54541: F, t54561: F, t9283: F, t9326: F) -> (F,) {
    let t55841 = 7.0 / 72.0 * t54531;
    let t55850 = 7.0 / 36.0 * t54535;
    let t55851 = 7.0 / 36.0 * t54537;
    let t55861 = t54519 / 24.0 + t54523 / 48.0 + t54529 / 24.0 + t55841 + 7.0 / 2304.0 * t51928 - t2408 * t9283 * t14185 * t9326 / 24.0 - t3066 * t29751 * t14882 / 8.0 - t55850 + t55851 + t54541 / 768.0 + t29775 * t14182 / 24.0 + t29775 * t14188 / 24.0 + t19895 * t15036 / 48.0 + t54561 / 48.0 - 35.0 / 216.0 * t52560;
    (t55861,)
}
