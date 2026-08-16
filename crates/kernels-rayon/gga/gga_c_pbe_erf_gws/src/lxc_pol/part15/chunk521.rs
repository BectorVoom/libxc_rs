//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 521/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk521(t2182: f64, t858: f64, t2210: f64, t884: f64, t2074: f64, t886: f64, t2089: f64, t2117: f64, t2126: f64, t2131: f64, t2140: f64, t2144: f64, t2152: f64, t2162: f64, t2166: f64, t2175: f64, t2194: f64, t2199: f64, t2204: f64, t2208: f64) -> (f64, f64, f64, f64, f64) {
    let t2211 = t858 * t2182;
    let t2212 = t2210 * t2211;
    let t2214 = t884 * t2212 / 16.0_f64;
    let t2215 = t858 * t2074;
    let t2216 = t886 * t2215;
    let t2218 = t884 * t2216 / 48.0_f64;
    let t2219 = t2089 + t2117 - t2126 + t2131 - t2140 - t2144 - t2152 + t2162 + t2166 + t2175 - t2194 - t2199 + t2204 + t2208 + t2214 - t2218;
    (t2212, t2214, t2216, t2218, t2219)
}
