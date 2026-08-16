//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1100/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1100(t1043: f64, t3153: f64, t3133: f64, t4982: f64, t3046: f64, t3286: f64, t3057: f64, t1071: f64, t1086: f64, t994: f64, t3316: f64, t989: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12131 = t1043 * t3153;
    let t12132 = t4982 * t3133;
    let t12146 = t3046 * t3286;
    let t12149 = t3057 * t3286;
    let t12153 = t1086 * t1071;
    let t12154 = t994 * t12153;
    let t12160 = t989 * t3316;
    (t12131, t12132, t12146, t12149, t12154, t12160)
}
