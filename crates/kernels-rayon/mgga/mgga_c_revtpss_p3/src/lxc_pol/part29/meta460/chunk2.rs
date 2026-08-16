//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1712/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1712(t25944: f64, t26277: f64, t25950: f64, t7515: f64, t213: f64, t7506: f64, t1445: f64, t2103: f64, t25909: f64, t26232: f64, t26235: f64, t26238: f64, t26241: f64, t26246: f64, t26251: f64, t26253: f64, t26257: f64, t26263: f64, t26266: f64, t26268: f64, t26272: f64, t26274: f64, t4132: f64, t7292: f64, t7295: f64, t7511: f64, t7532: f64) -> (f64, f64, f64, f64) {
    let t26279 = 0.17135234354032049604e-2_f64 * t25944 * t26277;
    let t26280 = t25950 * t7515;
    let t26282 = t213 * t7506;
    let t26291 = -0.14456046980341999104e-1_f64 * t26232 - 0.28912093960683998208e-1_f64 * t26235 - t26238 + 0.8673628188205199462e0_f64 * t7295 * t26241 + 0.4336814094102599731e0_f64 * t7295 * t26246 + t26251 + 0.19514881078765566038e-1_f64 * t26253 + 0.4336814094102599731e0_f64 * t7295 * t26257 - t26263 - 0.19514881078765566038e-1_f64 * t26266 + 0.25702851531048074406e-1_f64 * t26268 + 0.14456046980341999104e-1_f64 * t26272 - 0.25702851531048074406e-1_f64 * t26274 + t26279 - 0.25702851531048074406e-1_f64 * t26280 - 0.13170898365871023197e1_f64 * t26282 * t1445 - 0.4336814094102599731e0_f64 * t25909 * t2103 - 0.8673628188205199462e0_f64 * t7292 * t7532 - 0.65854491829355115987e0_f64 * t7511 * t4132;
    (t26279, t26280, t26282, t26291)
}
