//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1357/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1357(t54641: f64, t54681: f64, t1144: f64, t1206: f64, t14241: f64, t14311: f64, t14881: f64, t22263: f64, t3066: f64, t335: f64, t338: f64, t4083: f64, t51992: f64, t52542: f64, t52586: f64, t52589: f64, t52603: f64, t54649: f64, t54664: f64, t54675: f64, t8654: f64, t8793: f64, t9201: f64, t9283: f64, t9321: f64) -> f64 {
    let t55947 = 35.0_f64 / 216.0_f64 * t54641;
    let t55962 = 7.0_f64 / 36.0_f64 * t54681;
    let t55973 = t55947 - t335 * t338 * t9201 * t1206 / 96.0_f64 - 35.0_f64 / 216.0_f64 * t52586 - t335 * t338 * t1144 * t14241 / 96.0_f64 + t52589 - t54649 / 384.0_f64 - 7.0_f64 / 144.0_f64 * t51992 + t54664 / 12.0_f64 - t54675 / 12.0_f64 - 7.0_f64 / 72.0_f64 * t52603 - t55962 - t3066 * t9283 * t14881 * t9321 / 16.0_f64 + t8793 * t52542 / 24.0_f64 - t22263 * t4083 / 48.0_f64 - t8654 * t14311 / 48.0_f64;
    t55973
}
