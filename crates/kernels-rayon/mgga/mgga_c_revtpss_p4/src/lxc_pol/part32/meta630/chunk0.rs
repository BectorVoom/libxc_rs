//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2031/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2031(t30391: f64, t689: f64, t93314: f64, t93302: f64, t2718: f64, t7997: f64, t103212: f64, t103521: f64, t103529: f64, t103543: f64, t103547: f64, t106275: f64, t14587: f64, t1580: f64, t25383: f64, t26550: f64, t27353: f64, t28400: f64, t28425: f64, t30357: f64, t62604: f64, t62637: f64, t7420: f64, t7766: f64, t95945: f64, t95948: f64) -> f64 {
    let t110676 = t30391 * t689;
    let t110677 = t93314 * t110676;
    let t110679 = t93302 * t110676;
    let t110687 = t2718 * t7997;
    let t110694 = 0.4336814094102599731e0_f64 * t106275 * t7420 + 0.8673628188205199462e0_f64 * t25383 * t30357 - 0.8673628188205199462e0_f64 * t7766 * t28400 + 0.38549458614245330944e-1_f64 * t103521 - 0.13170898365871023197e1_f64 * t103212 * t1580 + 0.17135234354032049604e-2_f64 * t95945 - t103529 + 0.14456046980341999104e-1_f64 * t110677 - 0.25702851531048074406e-1_f64 * t110679 - 0.8673628188205199462e0_f64 * t27353 * t28425 * t62604 + 0.11565819519348392139e-2_f64 * t95948 + 0.86736281882051994624e-1_f64 * t103543 - 0.19274729307122665472e-1_f64 * t103547 - 0.17347256376410398924e1_f64 * t27353 * t110687 * t14587 + 0.4336814094102599731e0_f64 * t27353 * t26550 * t62637;
    t110694
}
