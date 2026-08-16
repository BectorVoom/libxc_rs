//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 945/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk945(t225: f64, t7908: f64, t2962: f64, t679: f64, t1049: f64, t1986: f64, t231: f64, t4910: f64, t7002: f64, t7007: f64, t7008: f64, t7009: f64, t7010: f64, t7013: f64, t7015: f64, t7017: f64, t7023: f64, t7026: f64, t7031: f64, t7033: f64) -> f64 {
    let t8400 = t7908 * t225;
    let t8404 = 8.0_f64 / 3.0_f64 * t2962 * t679;
    let t8405 = t1049 * t1986;
    let t8407 = t7002 - t7007 + t7008 + t4910 + t7009 + 4.0_f64 / 3.0_f64 * t8400 * t231 + t8404 + 4.0_f64 / 3.0_f64 * t8405 + t7010 - t7013 + t7015 - t7017 + t7023 - t7026 + t7031 - t7033;
    t8407
}
