//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1321/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1321(t1184: f64, t8975: f64, t51351: f64, t9509: f64, t51383: f64, t51401: f64, t54293: f64, t54294: f64, t54295: f64, t54297: f64, t54299: f64, t54302: f64, t54303: f64, t54305: f64) -> f64 {
    let t54307 = t1184 * t8975;
    let t54310 = t51351 * t9509;
    let t54312 = -7.0_f64 / 144.0_f64 * t51383 - t54293 - t54294 + t54295 / 48.0_f64 - t54297 / 24.0_f64 + t54299 / 48.0_f64 + t54302 + 5.0_f64 / 192.0_f64 * t54303 - 119.0_f64 / 3456.0_f64 * t54305 - t54307 / 48.0_f64 - 35.0_f64 / 576.0_f64 * t51401 + t54310 / 192.0_f64;
    t54312
}
