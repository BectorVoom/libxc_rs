//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2100/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2100(t25899: f64, t97966: f64, t25950: f64, t27888: f64, t25953: f64, t27884: f64, t13739: f64, t13743: f64, t25921: f64, t27896: f64, t28012: f64, t7279: f64, t7292: f64, t7926: f64, t94610: f64, t94761: f64, t94766: f64, t94769: f64, t94772: f64, t94774: f64, t94777: f64) -> f64 {
    let t97974 = 0.25702851531048074406e-1_f64 * t25899 * t97966;
    let t97976 = 0.25702851531048074406e-1_f64 * t25950 * t27888;
    let t97985 = t27884 * t25953;
    let t97994 = t97974 - t97976 - t94761 + 0.4336814094102599731e0_f64 * t94610 * t7926 - 0.25702851531048074406e-1_f64 * t94766 + 0.14456046980341999104e-1_f64 * t94769 - 0.68540937416128198418e-2_f64 * t94772 + 0.17347256376410398924e1_f64 * t25921 * t27896 - 0.25702851531048074406e-1_f64 * t94774 + 0.17135234354032049604e-1_f64 * t97985 - 0.39512695097613069591e1_f64 * t7279 * t13743 - 0.45699670022203476294e-2_f64 * t94777 - 0.8673628188205199462e0_f64 * t7292 * t28012 + 0.13170898365871023197e1_f64 * t7279 * t13739;
    t97994
}
