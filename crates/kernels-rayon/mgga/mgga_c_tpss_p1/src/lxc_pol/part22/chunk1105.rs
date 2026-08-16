//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1105/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1105(t4068: f64, t9507: f64, t4109: f64, t9504: f64, t1042: f64, t4105: f64, t2862: f64, t1519: f64, t2905: f64, t2863: f64, t4108: f64, t9292: f64) -> (f64, f64, f64, f64, f64) {
    let t12159 = 4.0_f64 * t9507 * t4068;
    let t12161 = 0.32163958997385070134e2_f64 * t9504 * t4109;
    let t12162 = t4105 * t1042;
    let t12164 = 4.0_f64 * t2862 * t12162;
    let t12165 = t1519 * t2905;
    let t12167 = 2.0_f64 * t2862 * t12165;
    let t12168 = t4108 * t2863;
    let t12170 = 0.96491876992155210402e2_f64 * t9292 * t12168;
    (t12159, t12161, t12164, t12167, t12170)
}
