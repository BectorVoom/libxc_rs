//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1577/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1577(t1011: f64, t1063: f64, t11656: f64, t11994: f64, t11999: f64, t16057: f64, t16062: f64, t16064: f64, t19930: f64, t19934: f64, t19940: f64, t19944: f64, t19947: f64, t3127: f64, t4837: f64, t6263: f64, t6312: f64) -> f64 {
    let t19950 = 0.15244095330869239812e-2_f64 * t11656 * t6263 + 0.11433071498151929859e-2_f64 * t11999 * t6312 + 0.85748036236139473944e-3_f64 * t1063 * t19930 - 0.57165357490759649296e-3_f64 * t1063 * t19934 - 0.28582678745379824648e-3_f64 * t11994 * t6263 - 0.28582678745379824648e-3_f64 * t3127 * t19940 + t16057 + t16062 - t16064 + 0.85748036236139473944e-3_f64 * t4837 * t19944 - t1011 * t19947 / 144.0_f64;
    t19950
}
