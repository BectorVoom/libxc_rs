//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1172/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1172<F: Float>(t11794: F, t14069: F, t14079: F, t3857: F, t54258: F, t54260: F, t57082: F, t57086: F, t57088: F, t57090: F, t57092: F, t57094: F, t57096: F, t57098: F, t57100: F, t11961: F, t14011: F) -> (F, F) {
    let t57102 = t11794 * t14069;
    let t57104 = t14079 * t3857;
    let t57106 = -t57082 / 768.0 + t57086 / 48.0 - t57088 / 24.0 - t57090 / 96.0 - t57092 / 768.0 - 5.0 / 192.0 * t57094 + t57096 / 96.0 + t57098 / 48.0 + t54258 + t57100 / 96.0 - t57102 / 96.0 - t54260 + 7.0 / 1152.0 * t57104;
    let t57108 = t14011 * t11961;
    (t57106, t57108)
}
