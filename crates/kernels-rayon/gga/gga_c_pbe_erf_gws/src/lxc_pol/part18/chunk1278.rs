//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1278/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1278(t13917: f64, t14583: f64, t53496: f64, t11375: f64, t13911: f64, t13925: f64, t15137: f64, t2376: f64, t27047: f64, t34850: f64, t35003: f64, t35193: f64, t36323: f64, t4002: f64, t51054: f64, t53012: f64, t53025: f64, t53028: f64, t53061: f64, t54928: f64, t56190: f64, t56194: f64, t56197: f64, t56199: f64, t56206: f64, t814: f64, t859: f64, t8629: f64, t892: f64) -> f64 {
    let t56209 = t13917 * t53496 * t14583;
    let t56223 = -t56190 / 48.0_f64 - t56194 / 384.0_f64 - t53012 + t54928 - t56197 / 192.0_f64 + t53025 - t53028 - t11375 * t27047 * t2376 * t56199 * t814 / 48.0_f64 - t56206 / 384.0_f64 + t56209 / 768.0_f64 - t53061 - t35193 * t4002 / 96.0_f64 + t8629 * t859 * t892 * t15137 / 96.0_f64 + t36323 * t13911 / 48.0_f64 - t35003 * t51054 / 48.0_f64 + t34850 * t13925 / 96.0_f64;
    t56223
}
