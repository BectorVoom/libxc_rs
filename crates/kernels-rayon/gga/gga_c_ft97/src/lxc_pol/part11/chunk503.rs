//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 503/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk503(t312: f64, t668: f64, t505: f64, t824: f64, t2874: f64, t309: f64, t848: f64) -> (f64, f64, f64, f64) {
    let t2875 = t312 * t668;
    let t2876 = t505 * t824;
    let t2877 = t2875 * t2876;
    let t2878 = t2874 * t2877;
    let t2881 = t848 * t309;
    (t2875, t2877, t2878, t2881)
}
