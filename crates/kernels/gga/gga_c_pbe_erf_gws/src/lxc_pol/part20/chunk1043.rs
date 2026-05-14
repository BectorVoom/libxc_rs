//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1043/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1043<F: Float>(t14797: F, t3068: F, t3990: F, t3989: F, t13888: F, t3060: F, t9283: F, t3070: F, t3965: F, t2409: F, t4155: F, t6781: F, t3062: F, t3959: F, t14128: F, t14130: F, t14777: F, t14779: F, t14782: F, t14785: F, t14788: F, t14793: F, t2408: F, t3066: F) -> (F, F, F, F, F) {
    let t14799 = t3990 * t14797 * t3068;
    let t14800 = t3989 * t14799;
    let t14802 = t13888 * t3060;
    let t14803 = t9283 * t14802;
    let t14806 = t3965 * t3070;
    let t14809 = t2409 * t6781 * t4155;
    let t14812 = t3959 * t3062;
    let t14814 = t14777 / 1536.0 + 7.0 / 288.0 * t14779 - t14782 / 96.0 - t14785 / 384.0 - t14788 / 96.0 - 7.0 / 288.0 * t14128 - t3066 * t14793 / 16.0 - 7.0 / 288.0 * t14130 + t14800 / 1536.0 - t2408 * t14803 / 24.0 + t14806 / 48.0 + t2408 * t14809 / 48.0 + t14812 / 48.0;
    (t14799, t14802, t14803, t14809, t14814)
}
