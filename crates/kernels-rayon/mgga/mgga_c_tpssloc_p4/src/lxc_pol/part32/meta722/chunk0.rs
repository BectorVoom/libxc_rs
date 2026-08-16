//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2299/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2299(t1184: f64, t6139: f64, t1716: f64, t1752: f64, t17686: f64, t2155: f64, t24589: f64, t24590: f64, t24601: f64, t24633: f64, t24638: f64, t254: f64, t27406: f64, t27412: f64, t27549: f64, t27747: f64, t27774: f64, t27775: f64, t27786: f64, t27799: f64, t29816: f64, t4945: f64, t6140: f64, t66860: f64, t7283: f64, t94349: f64, t94458: f64, t94503: f64, t94584: f64, t94676: f64) -> (f64, f64) {
    let t103422 = t6139 * t1184;
    let t103457 = 0.82246703342411321825e-2_f64 * t7283 * t6140 * t24638 - 0.82246703342411321825e-2_f64 * t7283 * t103422 * t27799 - t94676 + 0.16449340668482264365e-1_f64 * t7283 * t1716 * t94503 - 0.54831135561607547884e-2_f64 * t7283 * t24633 * t29816 + 0.16449340668482264365e-1_f64 * t7283 * t1716 * t94584 - 0.87729816898572076612e-1_f64 * t27406 * t27412 - t66860 * t2155 + 0.21932454224643019154e-1_f64 * t27549 * t24601 * t94349 * t17686 - 0.16449340668482264365e-1_f64 * t24589 * t24601 * t27774 * t17686 + 4.0_f64 * t4945 * t27747 - 12.0_f64 * t1752 * t254 * t27786 + 0.73108180748810063845e-2_f64 * t27549 * t94458 * t27775 + 0.54831135561607547884e-2_f64 * t24589 * t24590 * t29816;
    (t103422, t103457)
}
