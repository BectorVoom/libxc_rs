//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1193/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1193<F: Float>(t3074: F, t57512: F, t833: F, t15177: F, t3979: F, t15275: F, t840: F, t15213: F, t4414: F, t1185: F, t14419: F, t15272: F, t15325: F, t2376: F, t2408: F, t2409: F, t3066: F, t3067: F, t3068: F, t4155: F, t54480: F, t54482: F, t54598: F, t54599: F, t57495: F, t57497: F, t57500: F, t57506: F, t57509: F, t6781: F, t810: F, t8654: F, t938: F) -> (F,) {
    let t57514 = t3074 * t57512 * t833;
    let t57516 = t3979 * t15177;
    let t57518 = t840 * t15275;
    let t57534 = t4414 * t15213;
    let t57536 = t8654 * t1185 * t14419 / 24.0 + t57495 / 768.0 - t57497 / 96.0 - t57500 / 192.0 + t54598 * t54599 * t4155 * t3068 / 4.0 - t57506 / 48.0 - t57509 / 96.0 + t57514 / 96.0 + 7.0 / 4608.0 * t57516 + t54480 + t54482 + 7.0 / 288.0 * t57518 + t3066 * t2409 * t3067 * t15272 * t938 / 48.0 + t2408 * t2409 * t2376 * t15272 * t810 / 48.0 + t2408 * t2409 * t6781 * t15325 / 48.0 - 7.0 / 72.0 * t57534;
    (t57536,)
}
