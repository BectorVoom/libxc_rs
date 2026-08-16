//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2074/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2074(t25877: f64, t97699: f64, t25881: f64, t2028: f64, t25931: f64, t14224: f64, t689: f64, t25894: f64, t25921: f64, t25924: f64, t25966: f64, t26046: f64, t27837: f64, t27841: f64, t4131: f64, t7295: f64, t7920: f64, t94378: f64, t94388: f64, t94392: f64, t94399: f64, t97682: f64, t97687: f64, t97690: f64, t97698: f64) -> (f64, f64, f64) {
    let t97700 = t97699 * t25877;
    let t97702 = 0.28912093960683998208e-1_f64 * t97700 * t25881;
    let t97703 = t2028 * t25931;
    let t97705 = t14224 * t689;
    let t97707 = 0.14456046980341999104e-1_f64 * t25894 * t97703 * t97705;
    let t97716 = -t97682 + t97687 + t97690 - 0.26020884564615598386e1_f64 * t7295 * t25924 * t7920 * t4131 + 0.4336814094102599731e0_f64 * t27837 * t25966 - t97698 - t97702 - t97707 + 0.4336814094102599731e0_f64 * t27837 * t26046 - 0.19274729307122665471e-1_f64 * t94378 - 0.52041769129231196772e1_f64 * t25921 * t27841 - 0.34270468708064099208e-2_f64 * t94388 + 0.45699670022203476294e-2_f64 * t94392 + 0.28912093960683998208e-1_f64 * t94399;
    (t97703, t97705, t97716)
}
