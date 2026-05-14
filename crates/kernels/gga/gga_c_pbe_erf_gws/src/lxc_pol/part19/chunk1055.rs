//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1055/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1055<F: Float>(t14229: F, t14233: F, t14551: F, t14554: F, t14558: F, t14563: F, t15249: F, t15251: F, t15253: F, t15256: F, t15258: F, t15260: F, t15262: F, t15264: F, t15266: F, t15269: F) -> (F,) {
    let t15481 = -t15249 / 48.0 - t15251 / 192.0 + 5.0 / 192.0 * t15253 + t15256 / 24.0 + t15258 / 8.0 - 7.0 / 288.0 * t14551 - t15260 / 24.0 + t15262 / 48.0 + 7.0 / 72.0 * t14554 + 7.0 / 144.0 * t14558 + 7.0 / 36.0 * t14563 + t14229 + t14233 - t15264 / 96.0 + t15266 / 192.0 - t15269 / 48.0;
    (t15481,)
}
