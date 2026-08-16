//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1966/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1966(t26969: f64, t30767: f64, t2142: f64, t6744: f64, t7652: f64, t2138: f64, t6601: f64, t343: f64, t5842: f64, t136: f64, t1797: f64, t1808: f64, t26821: f64, t26844: f64, t26849: f64, t26867: f64, t26880: f64, t29020: f64, t29023: f64, t29027: f64, t29031: f64, t29034: f64, t29037: f64, t29065: f64, t29083: f64, t464: f64, t484: f64, t6619: f64, t6625: f64, t6631: f64, t6635: f64, t6640: f64, t6679: f64, t7618: f64, t7624: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t30768 = t26969 * t30767;
    let t30771 = t2142 * t6744;
    let t30772 = t7652 * t30771;
    let t30789 = t6601 * t2138;
    let t30799 = t5842 * t343;
    let t30800 = t30799 * t136;
    let t30805 = -0.45732285992607719436e-2_f64 * t29020 * t1797 + 0.57165357490759649296e-3_f64 * t29023 + 0.57165357490759649296e-3_f64 * t26880 * t6619 + 0.42874018118069736972e-3_f64 * t7618 * t6625 + 0.85748036236139473944e-3_f64 * t26844 * t6631 - 0.42874018118069736972e-3_f64 * t26849 * t6635 - t29027 / 54.0_f64 - t26821 - t29031 / 432.0_f64 - 0.3811023832717309953e-3_f64 * t29034 + 0.42874018118069736972e-3_f64 * t30789 * t484 + 0.30488190661738479624e-2_f64 * t29083 * t1808 - 0.28582678745379824648e-3_f64 * t7624 * t6679 - 0.57165357490759649296e-3_f64 * t29065 - 0.57165357490759649296e-3_f64 * t26867 * t6640 + 11.0_f64 / 108.0_f64 * t30800 * t464 - 0.57165357490759649296e-3_f64 * t29037 * t1808;
    (t30768, t30771, t30772, t30789, t30799, t30800, t30805)
}
