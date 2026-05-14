//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1086/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1086<F: Float>(t20154: F, t3067: F, t4164: F, t810: F, t14629: F, t4414: F, t14624: F, t9270: F, t14767: F, t2373: F, t1113: F, t13781: F, t2352: F, t3972: F, t824: F, t1115: F, t1185: F, t13772: F, t13849: F, t13910: F, t13929: F, t13939: F, t14576: F, t2074: F, t2182: F, t2376: F, t2408: F, t2409: F, t2498: F, t27105: F, t3066: F, t3207: F, t34963: F, t4182: F, t50967: F, t6793: F, t8654: F, t938: F) -> (F,) {
    let t53083 = t20154 * t3067 * t4164 * t810;
    let t53093 = 7.0 / 72.0 * t4414 * t14629;
    let t53099 = 7.0 / 72.0 * t9270 * t14624;
    let t53126 = t14767 * t2373;
    let t53131 = t3972 * t13781 * t1113 * t824 * t2352;
    let t53133 = -t6793 * t53083 / 12.0 + t8654 * t27105 * t13929 / 24.0 + t8654 * t1185 * t13910 / 24.0 - t53093 - t3066 * t2409 * t34963 * t13849 / 16.0 - t53099 + t2408 * t2409 * t2376 * t14576 * t810 / 24.0 + t2408 * t2409 * t2376 * t4182 * t2074 / 48.0 - t3207 * t2409 * t2376 * t4182 * t2182 / 16.0 + t3066 * t2409 * t3067 * t14576 * t938 / 24.0 - t2498 * t13939 / 48.0 - t1115 * t50967 / 96.0 - t2498 * t13772 / 48.0 - t53126 / 24.0 - t53131 / 1536.0;
    (t53133,)
}
