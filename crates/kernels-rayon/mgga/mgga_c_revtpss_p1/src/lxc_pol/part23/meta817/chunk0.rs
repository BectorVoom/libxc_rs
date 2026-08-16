//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2664/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2664(t1063: f64, t19662: f64, t3172: f64, t19667: f64, t11994: f64, t19920: f64, t19692: f64, t3127: f64, t19650: f64, t4837: f64, t19929: f64, t19933: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t65459 = t1063 * t3172 * t19662;
    let t65462 = t1063 * t3172 * t19667;
    let t65471 = t11994 * t19920;
    let t65488 = t3127 * t3172 * t19692;
    let t65493 = t4837 * t3172 * t19650;
    let t65507 = t1063 * t3172 * t19929;
    let t65510 = t1063 * t3172 * t19933;
    (t65459, t65462, t65471, t65488, t65493, t65507, t65510)
}
