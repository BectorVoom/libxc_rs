//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 791/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk791(t3153: f64, t3302: f64, t3154: f64, t3300: f64, t1043: f64, t1071: f64, t1089: f64, t3133: f64, t378: f64, t1035: f64, t3140: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3303 = t3153 * t3302;
    let t3304 = t3303 * t3154;
    let t3305 = t3300 * t3304;
    let t3309 = t1071 * t1043 * t1089;
    let t3313 = t378 * t3133 * t1089;
    let t3316 = t3140 * t1035;
    (t3303, t3304, t3305, t3309, t3313, t3316)
}
