//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1054/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1054(t1063: f64, t11988: f64, t1062: f64, t3196: f64, t3223: f64, t3229: f64, t369: f64, t361: f64, t351: f64, t3106: f64, t3111: f64, t3156: f64, t3172: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11989 = t1063 * t11988;
    let t11991 = t3196 * t1062;
    let t11994 = t3223 * t1062;
    let t12002 = t3229 * t369;
    let t12003 = t361 * t12002;
    let t12004 = t351 * t12003;
    let t12007 = t3106 * t3111;
    let t12009 = t3172 * t3156;
    (t11989, t11991, t11994, t12004, t12007, t12009)
}
