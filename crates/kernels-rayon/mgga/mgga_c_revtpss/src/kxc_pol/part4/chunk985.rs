//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 985/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk985(t10059: f64, t4086: f64, t543: f64, t2782: f64, t123: f64, t212: f64, t2434: f64, t4089: f64, t138: f64, t2438: f64, t785: f64, t1398: f64, t1419: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10065 = t4086 * t10059 * t543;
    let t10066 = t2782 * t10065;
    let t10069 = t123 * t2434 * t212;
    let t10070 = t10069 * t4089;
    let t10073 = t138 * t2438 * t785;
    let t10074 = t10073 * t4089;
    let t10079 = t4086 * t1419 * t1398 * t543;
    (t10066, t10069, t10070, t10073, t10074, t10079)
}
