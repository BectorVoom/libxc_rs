//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2096/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2096(t1358: f64, t212: f64, t27960: f64, t689: f64, t3923: f64, t7910: f64, t26050: f64, t27899: f64, t2453: f64, t27883: f64, t25946: f64, t27873: f64, t94890: f64) -> (f64, f64, f64, f64, f64) {
    let t97908 = 0.10975748638225852664e-1_f64 * t689 * t212 * t27960 * t1358;
    let t97909 = t7910 * t3923;
    let t97915 = 0.14456046980341999104e-1_f64 * t27899 * t26050;
    let t97916 = t2453 * t27883;
    let t97917 = t97916 * t25946;
    let t97920 = 0.28912093960683998208e-1_f64 * t94890 * t27873;
    (t97908, t97909, t97915, t97917, t97920)
}
