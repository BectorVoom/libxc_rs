//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2214/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2214(t1659: f64, t25576: f64, t27489: f64, t3111: f64, t11940: f64, t7131: f64, t16158: f64, t7132: f64, t1068: f64, t15719: f64, t1675: f64, t25577: f64, t3101: f64, t3204: f64, t4831: f64, t4839: f64, t93618: f64, t93620: f64, t93622: f64, t93627: f64, t93675: f64) -> f64 {
    let t100114 = t1659 * t25576;
    let t100117 = t27489 * t3111;
    let t100121 = t11940 * t7131;
    let t100132 = 0.3811023832717309953e-3_f64 * t7132 * t16158;
    let t100133 = 0.10162730220579493208e-2_f64 * t93618 - 0.30488190661738479624e-2_f64 * t93620 - 0.19055119163586549765e-3_f64 * t93622 + 0.28582678745379824648e-3_f64 * t93627 - 0.30488190661738479624e-2_f64 * t100114 * t1068 + 0.3811023832717309953e-3_f64 * t100117 - 0.57165357490759649296e-3_f64 * t27489 * t3101 - 0.25724410870841842183e-2_f64 * t100121 * t15719 - 0.91464571985215438873e-2_f64 * t3204 * t25576 * t4839 - 0.30488190661738479624e-2_f64 * t93675 * t1675 - 0.30488190661738479624e-2_f64 * t25577 * t4831 + t100132;
    t100133
}
