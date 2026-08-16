//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 515/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk515(t2182: f64, t339: f64, t2074: f64, t2100: f64, t2178: f64, t2181: f64, t340: f64, t870: f64, t871: f64, t343: f64) -> (f64, f64, f64, f64) {
    let t2183 = t339 * t2182;
    let t2186 = t339 * t2074;
    let t2189 = -t2100 * t339 * t340 + 6.0_f64 * t2178 * t871 - 12.0_f64 * t2181 * t2183 + 3.0_f64 * t2186 * t870;
    let t2190 = t2189 * t343;
    (t2183, t2186, t2189, t2190)
}
