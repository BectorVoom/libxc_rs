//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 772/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk772(t2842: f64, t2844: f64, t684: f64, t2881: f64, t2739: f64, t312: f64, t2874: f64, t2878: f64, t8392: f64, t2885: f64, t1934: f64, t875: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10452 = t2842 * t2844 * t684;
    let t10453 = t2881 * t10452;
    let t10457 = t312 * t2739 * t684;
    let t10458 = t2874 * t10457;
    let t10461 = t8392 * t2878;
    let t10463 = t8392 * t2885;
    let t10465 = t1934 * t875;
    (t10452, t10453, t10457, t10458, t10461, t10463, t10465)
}
