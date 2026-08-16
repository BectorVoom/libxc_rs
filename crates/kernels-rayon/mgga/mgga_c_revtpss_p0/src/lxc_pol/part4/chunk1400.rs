//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1400/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1400(t13142: f64, t17708: f64, t3601: f64, t3603: f64, t17710: f64, t3720: f64, t13127: f64, t471: f64, t17730: f64, t5046: f64, t12787: f64, t1260: f64, t5261: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t17747 = t13142 * t17708;
    let t17748 = t3601 * t3603;
    let t17749 = t17710 * t17748;
    let t17750 = t3720 * t17749;
    let t17753 = t13127 * t17708;
    let t17754 = t3601 * t471;
    let t17755 = t17710 * t17754;
    let t17756 = t3720 * t17755;
    let t17759 = t5046 * t17730;
    let t17760 = t12787 * t17759;
    let t17763 = t5261 * t1260;
    (t17747, t17750, t17753, t17756, t17760, t17763)
}
