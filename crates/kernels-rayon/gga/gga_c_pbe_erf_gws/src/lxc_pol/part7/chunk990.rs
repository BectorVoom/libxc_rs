//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 990/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk990(t16959: f64, t17047: f64, t17049: f64, t17051: f64, t17053: f64, t17055: f64, t17058: f64, t17063: f64, t17067: f64, t17069: f64, t17071: f64, t18209: f64) -> f64 {
    let t18211 = -t16959 + t17047 + t17049 - t17051 + t17053 + t17055 - t17058 + 8.0_f64 / 3.0_f64 * t18209 - t17063 + t17067 - t17069 - t17071;
    t18211
}
