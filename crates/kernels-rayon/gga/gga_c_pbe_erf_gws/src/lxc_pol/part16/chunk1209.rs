//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1209/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1209(t20154: f64, t3067: f64, t4088: f64, t938: f64, t20091: f64, t4090: f64, t14351: f64, t4414: f64, t1206: f64, t353: f64, t6161: f64, t859: f64) -> (f64, f64, f64, f64) {
    let t52154 = t20154 * t3067 * t4088 * t938;
    let t52159 = t20091 * t4090;
    let t52167 = t4414 * t14351;
    let t52179 = t859 * t353 * t1206 * t6161;
    (t52154, t52159, t52167, t52179)
}
