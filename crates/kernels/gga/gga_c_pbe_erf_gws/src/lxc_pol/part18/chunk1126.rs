//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1126/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1126<F: Float>(t14420: F, t26958: F, t11525: F, t51066: F, t53865: F, t15209: F, t8801: F, t2376: F, t2408: F, t2409: F, t3717: F, t4052: F, t53093: F, t53099: F, t53155: F, t53177: F, t53179: F, t53220: F, t56228: F, t56236: F, t56240: F, t56242: F, t56250: F, t8793: F) -> (F,) {
    let t56252 = t26958 * t14420;
    let t56255 = t53865 * t51066 * t11525;
    let t56257 = t8801 * t15209;
    let t56259 = t8793 * t53220 / 24.0 - 7.0 / 72.0 * t56228 + t2408 * t2409 * t2376 * t4052 * t3717 / 48.0 - t56236 / 12.0 + t56240 / 1536.0 - t53093 - t53099 + 7.0 / 1152.0 * t56242 - t53155 + t56250 / 384.0 - 7.0 / 72.0 * t56252 + 5.0 / 192.0 * t56255 + 7.0 / 48.0 * t56257 - t53177 - t53179;
    (t56259,)
}
