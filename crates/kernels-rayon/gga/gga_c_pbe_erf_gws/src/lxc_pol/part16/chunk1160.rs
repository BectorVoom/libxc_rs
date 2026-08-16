//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1160/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1160(t338: f64, t4228: f64, t892: f64, t14003: f64, t14115: f64, t14338: f64, t14345: f64, t14755: f64, t14768: f64, t14773: f64, t14777: f64, t14782: f64, t14785: f64, t14788: f64, t335: f64, t4083: f64, t8654: f64) -> (f64, f64) {
    let t15004 = t338 * t892 * t4228;
    let t15016 = t14755 / 768.0_f64 - t335 * t15004 / 96.0_f64 + t14338 + t14003 + t14115 + t14768 / 48.0_f64 - t14773 / 24.0_f64 - 7.0_f64 / 144.0_f64 * t14345 + t14777 / 768.0_f64 - t14782 / 48.0_f64 - t14785 / 192.0_f64 - t14788 / 48.0_f64 - t8654 * t4083 / 96.0_f64;
    (t15004, t15016)
}
