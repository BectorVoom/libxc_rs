//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 685/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk685(t1096: f64, t1976: f64, t7160: f64, t3140: f64, t378: f64, t1078: f64, t1982: f64, t1035: f64, t1043: f64, t1089: f64, t1984: f64, t359: f64, t7135: f64) -> (f64, f64, f64, f64, f64) {
    let t7161 = t1976 * t1096;
    let t7162 = t7160 * t7161;
    let t7165 = t378 * t3140;
    let t7166 = t7165 * t1078;
    let t7167 = t1982 * t7166;
    let t7168 = t1035 * t1976;
    let t7170 = t7168 * t1043 * t1089;
    let t7174 = t1984 * t359 * t7135;
    (t7162, t7167, t7168, t7170, t7174)
}
