//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1194/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1194<F: Float>(t1206: F, t12164: F, t15490: F, t2409: F, t3060: F, t3207: F, t335: F, t338: F, t36200: F, t36201: F, t4083: F, t4207: F, t53666: F, t55315: F, t56708: F, t56717: F, t56722: F, t56724: F, t56728: F, t56740: F, t56743: F, t56745: F, t56747: F, t6781: F, t9858: F) -> (F,) {
    let t58327 = -t56708 / 192.0 + t55315 - t56717 / 192.0 - t56722 / 768.0 - t9858 * t4083 / 96.0 + t56724 / 12.0 + 7.0 / 576.0 * t56728 - t53666 - t335 * t338 * t12164 * t1206 / 96.0 - t56740 / 48.0 + t36200 * t36201 * t4207 * t3060 / 4.0 - t56743 / 48.0 + 7.0 / 144.0 * t56745 - 7.0 / 1152.0 * t56747 - t3207 * t2409 * t6781 * t15490 / 16.0;
    (t58327,)
}
