//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1085/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1085<F: Float>(t1144: F, t13928: F, t4386: F, t14404: F, t20113: F, t50970: F, t50972: F, t51890: F, t53028: F, t53034: F, t53038: F, t53042: F, t53047: F, t53053: F, t53058: F, t53061: F, t53065: F, t53072: F, t6793: F, t8793: F) -> (F,) {
    let t53075 = t4386 * t1144 * t13928;
    let t53078 = -t53028 - t8793 * t51890 / 16.0 + t20113 * t14404 / 48.0 + t6793 * t53034 / 24.0 + t53038 / 192.0 + t6793 * t53042 / 24.0 + t6793 * t53047 / 24.0 + t53053 / 768.0 + t53058 / 384.0 - t53061 - t53065 / 768.0 + 7.0 / 72.0 * t50970 - 7.0 / 2304.0 * t50972 + t53072 / 192.0 + t6793 * t53075 / 24.0;
    (t53078,)
}
