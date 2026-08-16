//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1191/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1191(t1206: f64, t338: f64, t3907: f64, t14689: f64, t14708: f64, t14716: f64, t14745: f64, t14752: f64, t14989: f64, t14997: f64, t15283: f64, t15289: f64, t15297: f64, t15310: f64, t15312: f64, t15315: f64, t15318: f64, t15332: f64, t15335: f64, t335: f64) -> (f64, f64) {
    let t15513 = t338 * t3907 * t1206;
    let t15525 = t15283 / 192.0_f64 - 7.0_f64 / 72.0_f64 * t14689 + t15289 / 48.0_f64 + t15297 / 768.0_f64 - 7.0_f64 / 72.0_f64 * t14708 - 7.0_f64 / 576.0_f64 * t14716 + 7.0_f64 / 144.0_f64 * t14989 - t335 * t15513 / 96.0_f64 + 7.0_f64 / 36.0_f64 * t14745 + 5.0_f64 / 384.0_f64 * t15310 - t15312 / 24.0_f64 + 7.0_f64 / 144.0_f64 * t14997 - t15315 / 48.0_f64 + t15318 / 8.0_f64 + 7.0_f64 / 72.0_f64 * t14752 - t15332 / 12.0_f64 - t15335 / 24.0_f64;
    (t15513, t15525)
}
