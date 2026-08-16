//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1118/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1118(t5651: f64, t808: f64, t9736: f64, t241: f64, t820: f64, t9991: f64, t3923: f64, t9994: f64, t5673: f64, t5674: f64, t5697: f64, t9962: f64) -> (f64, f64, f64, f64, f64) {
    let t13800 = t808 * t5651;
    let t13801 = t9736 * t13800;
    let t13804 = t820 * t9991 * t241;
    let t13805 = t9994 * t3923;
    let t13807 = t5673 * t5674 * t13805;
    let t13810 = t9962 * t5697;
    (t13801, t13804, t13805, t13807, t13810)
}
