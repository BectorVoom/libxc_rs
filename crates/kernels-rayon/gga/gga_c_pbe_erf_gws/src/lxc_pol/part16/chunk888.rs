//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 888/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk888(t1017: f64, t1764: f64, t1403: f64, t1827: f64, t587: f64, t1000: f64, t1406: f64, t1821: f64, t1820: f64, t197: f64, t2620: f64, t7355: f64) -> (f64, f64, f64, f64) {
    let t7685 = t1017 * t1764;
    let t7686 = t7685 * t1403;
    let t7687 = t1827 * t7686;
    let t7689 = 8.0_f64 / 45.0_f64 * t587 * t7687;
    let t7690 = t1000 * t1406;
    let t7691 = t1821 * t7690;
    let t7693 = 8.0_f64 / 45.0_f64 * t1820 * t7691;
    let t7694 = t2620 * t197;
    let t7695 = t7694 * t7355;
    (t7689, t7693, t7694, t7695)
}
