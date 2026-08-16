//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 955/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk955(t498: f64, t9904: f64, t3205: f64, t3387: f64, t1186: f64, t3214: f64, t177: f64, t3297: f64, t737: f64, t30: f64, t490: f64, t33: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9906 = 120.0_f64 * t9904 * t498;
    let t9909 = t3387 * t3205;
    let t9913 = t3214 * t1186;
    let t9915 = t3297 * t177;
    let t9916 = t9915 * t737;
    let t9922 = t30 * t30;
    let t9924 = 1.0_f64 / t490 / t9922;
    let t9934 = t33 * t33;
    (t9906, t9909, t9913, t9916, t9924, t9934)
}
