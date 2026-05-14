//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1175/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1175<F: Float>(t54491: F, t14954: F, t4414: F, t14981: F, t15004: F, t840: F, t54504: F, t1105: F, t12213: F, t14200: F, t14240: F, t14272: F, t15081: F, t2376: F, t2408: F, t2409: F, t2494: F, t3066: F, t3067: F, t3306: F, t4110: F, t54496: F, t54502: F, t54508: F, t54512: F, t8589: F, t938: F) -> (F,) {
    let t55796 = 7.0 / 1152.0 * t54491;
    let t55807 = 7.0 / 72.0 * t4414 * t14954;
    let t55809 = 7.0 / 72.0 * t4414 * t14981;
    let t55831 = 7.0 / 144.0 * t840 * t15004;
    let t55833 = 7.0 / 72.0 * t54504;
    let t55836 = t55796 + t2408 * t2409 * t8589 * t14200 / 48.0 + t3066 * t2409 * t3067 * t15081 * t938 / 24.0 - t55807 - t55809 + t3066 * t2409 * t12213 * t14272 / 24.0 + t2408 * t2409 * t2376 * t4110 * t2494 / 24.0 + t3066 * t2409 * t3067 * t4110 * t3306 / 24.0 + t2408 * t2409 * t2376 * t14240 * t1105 / 48.0 - t54496 / 12.0 + t55831 - t54502 / 384.0 + t55833 + t54508 / 192.0 + t54512 / 384.0;
    (t55836,)
}
