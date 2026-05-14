//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1142/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1142<F: Float>(t14135: F, t3912: F, t51913: F, t11505: F, t3972: F, t3975: F, t15288: F, t2397: F, t14757: F, t2408: F, t2409: F, t3066: F, t3067: F, t3886: F, t4052: F, t53546: F, t53578: F, t53584: F, t53585: F, t53598: F, t56578: F, t56582: F, t56586: F, t56588: F, t56590: F, t8589: F) -> (F,) {
    let t56593 = t3912 * t14135 * t51913;
    let t56596 = t3972 * t3975 * t11505;
    let t56599 = t15288 * t2397;
    let t56601 = t3066 * t2409 * t3067 * t4052 * t3886 / 48.0 + t2408 * t2409 * t8589 * t14757 / 24.0 + t56578 / 96.0 + t56582 / 768.0 - t56586 / 384.0 - t53546 + t56588 / 96.0 + t56590 / 96.0 + t56593 / 48.0 - t53578 + t56596 / 1536.0 - t53584 + 35.0 / 108.0 * t53585 - t53598 + t56599 / 96.0;
    (t56601,)
}
