//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1187/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1187<F: Float>(t1118: F, t1133: F, t361: F, t3223: F, t50998: F, t12239: F, t14121: F, t11409: F, t3965: F, t12250: F, t3959: F, t2409: F, t35433: F, t51870: F, t51877: F, t53784: F, t53971: F, t53976: F, t53977: F, t53980: F, t53986: F, t54430: F, t55751: F, t8793: F) -> (F,) {
    let t57384 = t361 * t1118 * t1133;
    let t57386 = t50998 * t57384 * t3223;
    let t57390 = t14121 * t12239;
    let t57393 = t3965 * t11409;
    let t57395 = t3959 * t12250;
    let t57398 = t3959 * t2409 * t35433;
    let t57401 = t57386 / 192.0 - t8793 * t53784 / 8.0 - t53971 + t53976 - t57390 / 16.0 - 35.0 / 216.0 * t53977 + t57393 / 24.0 + t53980 + t53986 + t57395 / 48.0 - t55751 + t54430 - t57398 / 48.0 - t51870 + 35.0 / 432.0 * t51877;
    (t57401,)
}
