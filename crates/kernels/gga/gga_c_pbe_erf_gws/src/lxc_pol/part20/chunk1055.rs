//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1055/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1055<F: Float>(t15247: F, t15271: F, t898: F, t338: F, t353: F, t3862: F, t3975: F, t3972: F, t13780: F, t3742: F, t3990: F, t13859: F, t13895: F, t14931: F, t14962: F, t14974: F, t15187: F, t15192: F, t15195: F, t15198: F, t15201: F, t15205: F, t15209: F, t15213: F, t15216: F, t2408: F, t3066: F, t3207: F, t335: F, t3913: F, t4002: F) -> (F, F, F, F, F, F) {
    let t15272 = t15247 + t15271;
    let t15273 = t898 * t15272;
    let t15275 = t338 * t353 * t15273;
    let t15278 = t3975 * t3862;
    let t15279 = t3972 * t15278;
    let t15282 = t3990 * t13780 * t3742;
    let t15283 = t13859 * t15282;
    let t15285 = t15187 / 1536.0 - t3913 * t4002 / 96.0 - t15192 / 192.0 + t3066 * t15195 / 24.0 + t14931 + t15198 / 24.0 + t15201 / 768.0 - t15205 / 768.0 - t3207 * t15209 / 16.0 + t2408 * t15213 / 24.0 + t15216 / 48.0 + t13895 - t335 * t15275 / 96.0 + t14962 + t15279 / 1536.0 + t15283 / 384.0 - t14974;
    (t15272, t15273, t15275, t15278, t15282, t15285)
}
