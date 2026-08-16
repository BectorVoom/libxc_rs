//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 454/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk454(t20: f64, t711: f64, t245: f64, t671: f64, t156: f64, t670: f64, t226: f64, t678: f64, t230: f64, t666: f64, t131: f64, t137: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2003 = t711 * t20;
    let t2004 = t245 * t671;
    let t2006 = 0.11181742741110338156e-1_f64 * t2003 * t2004;
    let t2007 = t156 * t671;
    let t2009 = 0.72140275749098955847e-1_f64 * t670 * t2007;
    let t2014 = 8.0_f64 / 3.0_f64 * t226 * t678;
    let t2015 = t666 * t230;
    let t2029 = t131 * t131;
    let t2030 = 1.0_f64 / t2029;
    let t2031 = t2030 * t137;
    (t2003, t2004, t2006, t2007, t2009, t2014, t2015, t2029, t2030, t2031)
}
