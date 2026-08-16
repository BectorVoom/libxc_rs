//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 871/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk871(t13233: f64, t13235: f64, t13237: f64, t13238: f64, t13240: f64, t13245: f64, t13247: f64, t13284: f64, t13295: f64, t13302: f64, t13306: f64, t13308: f64, t13313: f64) -> f64 {
    let t13671 = -t13233 - t13235 - t13237 + t13238 - t13240 + t13245 + t13247 + t13284 + t13295 + t13302 - t13306 - t13308 - t13313;
    t13671
}
