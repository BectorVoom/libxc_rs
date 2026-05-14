//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1106/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1106<F: Float>(t3287: F, t51255: F, t3142: F, t51382: F, t1125: F, t51292: F, t14024: F, t3120: F, t21296: F, t367: F, t899: F, t3237: F, t51371: F, t3242: F, t3232: F, t14079: F, t3283: F) -> (F, F, F, F, F, F, F, F, F) {
    let t54257 = t51255 * t3287;
    let t54258 = 7.0 / 144.0 * t54257;
    let t54259 = t51382 * t3142;
    let t54260 = 7.0 / 72.0 * t54259;
    let t54267 = t1125 * t51292;
    let t54268 = 7.0 / 72.0 * t54267;
    let t54271 = t3120 * t14024;
    let t54272 = 7.0 / 144.0 * t54271;
    let t54279 = t899 * t21296 * t367;
    let t54283 = t51371 * t3237;
    let t54284 = 7.0 / 576.0 * t54283;
    let t54285 = t51371 * t3242;
    let t54286 = 7.0 / 144.0 * t54285;
    let t54289 = t51371 * t3232;
    let t54290 = 7.0 / 144.0 * t54289;
    let t54301 = t14079 * t3283;
    (t54258, t54260, t54268, t54272, t54279, t54284, t54286, t54290, t54301)
}
