//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 194/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk194(t746: f64, t747: f64, t741: f64, t737: f64, t724: f64, t571: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t748 = t746 * t747;
    let t749 = t741 * t748;
    let t751 = 1.0_f64 + t737 / 16.0_f64 - t749 / 256.0_f64;
    let t752 = 1.0_f64 / t751;
    let t753 = t724 * t752;
    let t755 = 1.0_f64 + 0.5137e-1_f64 * t571;
    (t748, t749, t751, t752, t753, t755)
}
