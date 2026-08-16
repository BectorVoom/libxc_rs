//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 836/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk836(t267: f64, t4872: f64, t4873: f64, t4876: f64, t4910: f64, t6971: f64, t6995: f64, t6998: f64, t7002: f64, t7007: f64, t7008: f64, t7009: f64, t7010: f64, t7013: f64, t7015: f64, t7017: f64, t7023: f64) -> f64 {
    let t7024 = -t4872 - t6971 - t6995 * t267 / 15.0_f64 + 2.0_f64 / 135.0_f64 * t6998 + 0.66490888888888888888e-1_f64 * t4873 + t4876 + t7002 - t7007 + t7008 + t4910 + t7009 + t7010 - t7013 + t7015 - t7017 + t7023;
    t7024
}
