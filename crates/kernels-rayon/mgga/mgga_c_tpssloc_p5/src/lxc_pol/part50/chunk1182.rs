//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1182/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1182(t22893: f64, t23164: f64, t32818: f64, t113016: f64, t118616: f64, t118677: f64, t118679: f64, t118682: f64, t118684: f64, t118694: f64, t118695: f64, t118699: f64, t118700: f64, t118705: f64, t118710: f64, t118715: f64, t118719: f64, t118725: f64, t1510: f64, t226: f64, t235: f64, t30694: f64, t32850: f64, t4182: f64, t4234: f64, t4281: f64, t4291: f64, t808: f64, t812: f64, t829: f64) -> f64 {
    let t118727 = t23164 * t22893 * t32818;
    let t118728 = 0.16449340668482264365e-1_f64 * t118727;
    let t118729 = -t113016 * t1510 * t812 + t118616 * t226 * t235 + 2.0_f64 * t118684 * t4182 * t4281 - t118684 * t4291 * t829 - t118705 * t812 * t829 - t30694 * t4234 * t812 + t32850 * t808 + t118677 + t118679 + t118682 + t118694 + t118695 + t118699 + t118700 - t118710 - t118715 + t118719 - t118725 + t118728;
    t118729
}
