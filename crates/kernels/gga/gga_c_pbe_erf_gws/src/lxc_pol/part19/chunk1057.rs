//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1057/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1057<F: Float>(t1206: F, t338: F, t3907: F, t14689: F, t14708: F, t14716: F, t14745: F, t14752: F, t14989: F, t14997: F, t15283: F, t15289: F, t15297: F, t15310: F, t15312: F, t15315: F, t15318: F, t15332: F, t15335: F, t335: F) -> (F, F) {
    let t15513 = t338 * t3907 * t1206;
    let t15525 = t15283 / 192.0 - 7.0 / 72.0 * t14689 + t15289 / 48.0 + t15297 / 768.0 - 7.0 / 72.0 * t14708 - 7.0 / 576.0 * t14716 + 7.0 / 144.0 * t14989 - t335 * t15513 / 96.0 + 7.0 / 36.0 * t14745 + 5.0 / 384.0 * t15310 - t15312 / 24.0 + 7.0 / 144.0 * t14997 - t15315 / 48.0 + t15318 / 8.0 + 7.0 / 72.0 * t14752 - t15332 / 12.0 - t15335 / 24.0;
    (t15513, t15525)
}
