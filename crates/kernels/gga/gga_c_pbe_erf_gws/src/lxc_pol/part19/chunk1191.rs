//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1191/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1191<F: Float>(t1206: F, t338: F, t3907: F, t14689: F, t14708: F, t14716: F, t14745: F, t14752: F, t14989: F, t14997: F, t15283: F, t15289: F, t15297: F, t15310: F, t15312: F, t15315: F, t15318: F, t15332: F, t15335: F, t335: F) -> (F, F) {
    let t15513 = t338 * t3907 * t1206;
    let t15525 = t15283 / F::new(192.0) - F::new(7.0) / F::new(72.0) * t14689 + t15289 / F::new(48.0) + t15297 / F::new(768.0) - F::new(7.0) / F::new(72.0) * t14708 - F::new(7.0) / F::new(576.0) * t14716 + F::new(7.0) / F::new(144.0) * t14989 - t335 * t15513 / F::new(96.0) + F::new(7.0) / F::new(36.0) * t14745 + F::new(5.0) / F::new(384.0) * t15310 - t15312 / F::new(24.0) + F::new(7.0) / F::new(144.0) * t14997 - t15315 / F::new(48.0) + t15318 / F::new(8.0) + F::new(7.0) / F::new(72.0) * t14752 - t15332 / F::new(12.0) - t15335 / F::new(24.0);
    (t15513, t15525)
}
