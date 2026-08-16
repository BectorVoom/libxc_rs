//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 401/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk401(t1243: f64, t1251: f64, t1365: f64, t153: f64, t532: f64) -> f64 {
    let t1368 = 0.23e-2_f64 * t1243 + 0.22758333333333333333e-1_f64 * t1251 - 0.60972258698505103132e-2_f64 * t532 + 0.10844166666666666667e-2_f64 * t153 * t1365;
    t1368
}
