//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1087/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1087(t12031: f64, t12034: f64, t12038: f64, t12040: f64, t12047: f64, t12056: f64, t12060: f64, t12067: f64, t12071: f64, t12078: f64, t12082: f64, t12086: f64, t12093: f64) -> f64 {
    let t12160 = -t12031 + t12034 + t12038 - t12040 + t12047 - t12056 + t12060 - t12067 - t12071 + t12078 + t12082 - t12086 - t12093;
    t12160
}
