//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2213/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2213(t16035: f64, t25580: f64, t25569: f64, t4817: f64, t100019: f64, t15592: f64, t15622: f64, t15847: f64, t25517: f64, t3317: f64, t4783: f64, t4831: f64, t4902: f64, t4912: f64, t7132: f64, t93543: f64, t93597: f64, t93602: f64, t93611: f64, t93616: f64, t93667: f64) -> f64 {
    let t100092 = 0.57165357490759649296e-3_f64 * t25580 * t16035;
    let t100097 = 0.3811023832717309953e-3_f64 * t25569 * t4817;
    let t100109 = -0.85748036236139473944e-3_f64 * t93543 * t4912 - 0.3811023832717309953e-3_f64 * t93602 - t100092 + 0.45732285992607719436e-2_f64 * t3317 * t100019 * t4902 + t100097 + 0.57165357490759649296e-3_f64 * t25569 * t4831 + 0.28582678745379824648e-3_f64 * t7132 * t15847 + 0.17149607247227894789e-2_f64 * t93667 * t15622 + 0.28582678745379824648e-3_f64 * t25517 * t15592 - 0.30488190661738479624e-2_f64 * t93597 * t4783 + t93611 + 0.96545937095505185476e-2_f64 * t93616;
    t100109
}
