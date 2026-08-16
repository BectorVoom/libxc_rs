//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1042/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1042(t11643: f64, t3127: f64, t3135: f64, t3172: f64, t1041: f64, t1024: f64, t3105: f64, t3151: f64, t3153: f64, t1052: f64, t360: f64, t3089: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11644 = t3127 * t11643;
    let t11648 = t3172 * t3135;
    let t11649 = t1041 * t11648;
    let t11656 = t1024 * t3105;
    let t11659 = t3151 * t3153;
    let t11670 = t360 * t1052;
    let t11671 = t11670 * t3089;
    (t11644, t11649, t11656, t11659, t11670, t11671)
}
