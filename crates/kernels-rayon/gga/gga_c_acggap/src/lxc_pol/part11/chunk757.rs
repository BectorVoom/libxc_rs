//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 757/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk757(t157: f64, t2122: f64, t406: f64, t2152: f64, t1960: f64, t880: f64, t309: f64, t314: f64, t180: f64, t621: f64, t609: f64, t851: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7916 = t2122 * t406 * t157;
    let t7917 = t2152 * t7916;
    let t7921 = 0.65854491829355115987e0_f64 * t1960 * t880;
    let t7922 = t309 * t309;
    let t7923 = t7922 * t314;
    let t7924 = t7923 * t180;
    let t7926 = 0.8673628188205199462e0_f64 * t7924 * t621;
    let t7927 = t851 * t609;
    (t7917, t7921, t7922, t7923, t7924, t7926, t7927)
}
