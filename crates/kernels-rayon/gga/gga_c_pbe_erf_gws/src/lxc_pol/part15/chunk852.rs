//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 852/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk852(t2576: f64, t4913: f64, t2722: f64, t626: f64, t422: f64, t1815: f64, t639: f64, t5357: f64, t561: f64, t213: f64, t174: f64, t838: f64) -> (f64, f64, f64, f64) {
    let t7223 = 16.0_f64 / 45.0_f64 * t4913 * t2576;
    let t7224 = t2722 * t626;
    let t7225 = t7224 * t422;
    let t7226 = t1815 * t7225;
    let t7228 = 8.0_f64 / 45.0_f64 * t639 * t7226;
    let t7230 = 4.0_f64 / 15.0_f64 * t561 * t5357;
    let t7231 = t213 * t626;
    let t7233 = t174 * t838 * t7231;
    (t7223, t7228, t7230, t7233)
}
