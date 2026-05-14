//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 954/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk954<F: Float>(t26270: F, t686: F, t7284: F, t7289: F, t136: F, t2102: F, t2457: F, t25944: F, t25950: F, t7515: F, t213: F, t7506: F, t1445: F, t2103: F, t25909: F, t26232: F, t26235: F, t26238: F, t26241: F, t26246: F, t26251: F, t26253: F, t26257: F, t26263: F, t26266: F, t26268: F, t4132: F, t7292: F, t7295: F, t7511: F, t7532: F) -> (F, F, F, F, F) {
    let t26271 = t26270 * t686;
    let t26272 = t7284 * t26271;
    let t26274 = t7289 * t26271;
    let t26276 = t2102 * t136;
    let t26277 = t26276 * t2457;
    let t26279 = 0.17135234354032049604e-2 * t25944 * t26277;
    let t26280 = t25950 * t7515;
    let t26282 = t213 * t7506;
    let t26291 = -0.14456046980341999104e-1 * t26232 - 0.28912093960683998208e-1 * t26235 - t26238 + 0.8673628188205199462e0 * t7295 * t26241 + 0.4336814094102599731e0 * t7295 * t26246 + t26251 + 0.19514881078765566038e-1 * t26253 + 0.4336814094102599731e0 * t7295 * t26257 - t26263 - 0.19514881078765566038e-1 * t26266 + 0.25702851531048074406e-1 * t26268 + 0.14456046980341999104e-1 * t26272 - 0.25702851531048074406e-1 * t26274 + t26279 - 0.25702851531048074406e-1 * t26280 - 0.13170898365871023197e1 * t26282 * t1445 - 0.4336814094102599731e0 * t25909 * t2103 - 0.8673628188205199462e0 * t7292 * t7532 - 0.65854491829355115987e0 * t7511 * t4132;
    (t26271, t26276, t26277, t26282, t26291)
}
