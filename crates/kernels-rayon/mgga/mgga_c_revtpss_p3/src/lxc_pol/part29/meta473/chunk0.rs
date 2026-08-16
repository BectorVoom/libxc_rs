//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1738/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1738(t26563: f64, t789: f64, t231: f64, t7398: f64, t836: f64, t7076: f64, t2061: f64, t2645: f64, t2062: f64, t2453: f64, t2458: f64, t2067: f64, t25383: f64, t25391: f64, t25407: f64, t26529: f64, t26534: f64, t26536: f64, t26538: f64, t26541: f64, t26545: f64, t26547: f64, t26551: f64, t26557: f64, t26558: f64, t26561: f64, t2772: f64, t2829: f64, t7070: f64, t7403: f64, t7420: f64, t887: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t26564 = t26563 * t789;
    let t26567 = t7398 * t836 * t231;
    let t26568 = t7076 * t26567;
    let t26573 = t7076 * t2061 * t2645 * t231;
    let t26576 = t2453 * t2062;
    let t26578 = 0.11565819519348392139e-2_f64 * t26576 * t2458;
    let t26579 = 0.8673628188205199462e0_f64 * t25383 * t7420 - 0.65854491829355115987e0_f64 * t7403 * t2829 + 0.14456046980341999104e-1_f64 * t26529 + 0.13170898365871023197e1_f64 * t7403 * t2772 - t26534 - t26536 - t26538 - 0.4336814094102599731e0_f64 * t25407 * t2067 - 0.28912093960683998208e-1_f64 * t26541 + 0.14456046980341999104e-1_f64 * t26545 - 0.13170898365871023197e1_f64 * t26547 * t887 - 0.17347256376410398924e1_f64 * t25391 * t26551 - t26557 - 0.25702851531048074406e-1_f64 * t26558 + 0.10975748638225852664e-1_f64 * t26561 + 0.19514881078765566038e-1_f64 * t26564 + 0.8673628188205199462e0_f64 * t7070 * t26568 + 0.4336814094102599731e0_f64 * t7070 * t26573 + t26578;
    (t26564, t26568, t26573, t26576, t26578, t26579)
}
