//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2230/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2230(t104707: f64, t1285: f64, t12987: f64, t7623: f64, t5261: f64, t1230: f64, t29082: f64, t29037: f64, t3636: f64, t104647: f64, t1266: f64, t17265: f64, t17347: f64, t17369: f64, t17732: f64, t29040: f64, t3631: f64, t3640: f64, t3644: f64, t7624: f64, t97169: f64) -> f64 {
    let t104721 = t1285 * t104707;
    let t104727 = t12987 * t7623;
    let t104732 = t5261 * t7623;
    let t104739 = t1230 * t29082;
    let t104742 = t29037 * t3636;
    let t104746 = 0.30488190661738479624e-2_f64 * t104721 * t3631 - 0.28582678745379824648e-3_f64 * t97169 - 0.28582678745379824648e-3_f64 * t7624 * t17369 - 0.25724410870841842183e-2_f64 * t104727 * t17347 + 0.85748036236139473944e-3_f64 * t29040 * t17265 - 0.57165357490759649296e-3_f64 * t104732 * t1266 - 0.28582678745379824648e-3_f64 * t29037 * t3640 - 0.57165357490759649296e-3_f64 * t29037 * t3644 + 0.30488190661738479624e-2_f64 * t104739 * t1266 - 0.3811023832717309953e-3_f64 * t104742 + 0.11433071498151929859e-2_f64 * t104647 * t17732;
    t104746
}
