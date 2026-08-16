//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1226/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1226(t4002: f64, t4424: f64, t1477: f64, t274: f64, t833: f64, t850: f64, t851: f64, t1172: f64, t1198: f64, t319: f64, t12276: f64, t50832: f64) -> (f64, f64, f64, f64, f64) {
    let t52020 = t4424 * t4002;
    let t52033 = t274 * t1477;
    let t52036 = t850 * t851 * t52033 * t833;
    let t52774 = t1172 * t319 * t1198;
    let t52789 = 6.0_f64 * t50832 * t12276;
    (t52020, t52033, t52036, t52774, t52789)
}
