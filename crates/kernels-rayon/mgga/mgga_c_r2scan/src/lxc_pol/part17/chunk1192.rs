//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1192/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1192(t1054: f64, t6132: f64, t8745: f64, t6139: f64, t8741: f64, t39613: f64, t40195: f64, t8752: f64, t39614: f64, t5108: f64, t9481: f64, t6106: f64, t8756: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t43622 = t6132 * t1054 * t8745;
    let t43625 = t6139 * t1054 * t8741;
    let t43631 = t39613 * t40195 * t8752;
    let t43635 = t39613 * t39614 * t8741;
    let t43638 = t5108 * t1054 * t9481;
    let t43641 = t6106 * t1054 * t8756;
    (t43622, t43625, t43631, t43635, t43638, t43641)
}
