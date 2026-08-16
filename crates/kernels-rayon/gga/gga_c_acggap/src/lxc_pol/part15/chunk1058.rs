//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1058/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1058(t35850: f64, t35909: f64, t35911: f64, t35913: f64, t35915: f64, t35917: f64, t35919: f64, t35926: f64, t35930: f64, t35934: f64, t35951: f64, t35961: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t37744 = 0.1120625e0_f64 * t35850;
    let t37777 = 0.916875e-1_f64 * t35909;
    let t37778 = 0.916875e-1_f64 * t35911;
    let t37779 = 0.61125e-1_f64 * t35913;
    let t37780 = 0.61125e-1_f64 * t35915;
    let t37781 = 0.34299214494455789578e-2_f64 * t35917;
    let t37782 = 0.34299214494455789578e-2_f64 * t35919;
    let t37787 = 0.64025200389650807212e-1_f64 * t35926;
    let t37789 = 0.85748036236139473944e-3_f64 * t35930;
    let t37790 = 0.42874018118069736972e-3_f64 * t35934;
    let t37801 = 0.34299214494455789578e-2_f64 * t35951;
    let t37807 = 0.34299214494455789578e-2_f64 * t35961;
    (t37744, t37777, t37778, t37779, t37780, t37781, t37782, t37787, t37789, t37790, t37801, t37807)
}
