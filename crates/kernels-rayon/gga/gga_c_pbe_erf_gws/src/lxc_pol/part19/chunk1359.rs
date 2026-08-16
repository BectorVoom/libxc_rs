//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1359/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1359(t11889: f64, t1206: f64, t14327: f64, t3207: f64, t3913: f64, t53334: f64, t55031: f64, t55036: f64, t55059: f64, t55062: f64, t56299: f64, t56302: f64, t56305: f64, t56307: f64, t56309: f64, t56312: f64, t56316: f64, t56318: f64, t56321: f64, t9283: f64) -> f64 {
    let t58083 = -t55031 + t56299 / 256.0_f64 - t3207 * t9283 * t1206 * t11889 / 8.0_f64 - t3913 * t14327 / 96.0_f64 + t56302 / 768.0_f64 + t56305 / 192.0_f64 - t56307 / 24.0_f64 - t56309 / 12.0_f64 - 5.0_f64 / 192.0_f64 * t56312 - t56316 / 48.0_f64 - t56318 / 12.0_f64 - t56321 / 48.0_f64 + t55036 + 35.0_f64 / 108.0_f64 * t55059 - t55062 - 119.0_f64 / 3456.0_f64 * t53334;
    t58083
}
