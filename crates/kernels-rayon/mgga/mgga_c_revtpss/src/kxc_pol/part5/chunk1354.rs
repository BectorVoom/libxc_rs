//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1354/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1354(t21512: f64, t5480: f64, t1280: f64, t20747: f64, t5230: f64, t5486: f64, t21342: f64, t489: f64, t1248: f64, t1287: f64, t6695: f64, t1774: f64, t17821: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t21518 = t21512 * t5480;
    let t21521 = t1280 * t20747;
    let t21524 = t5486 * t5230;
    let t21527 = t489 * t21342;
    let t21535 = t6695 * t1248 * t1287;
    let t21538 = t17821 * t1774;
    (t21518, t21521, t21524, t21527, t21535, t21538)
}
