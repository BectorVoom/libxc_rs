//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2135/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2135(t213: f64, t29636: f64, t231: f64, t6048: f64, t836: f64, t6071: f64, t106111: f64, t106172: f64, t106275: f64, t14587: f64, t1579: f64, t1956: f64, t1957: f64, t233: f64, t25383: f64, t25391: f64, t25392: f64, t27353: f64, t27354: f64, t27357: f64, t29611: f64, t29698: f64, t62628: f64, t7048: f64, t7070: f64, t7071: f64, t7073: f64, t7083: f64, t887: f64, t93286: f64, t93349: f64, t99366: f64, t99375: f64, t99381: f64) -> f64 {
    let t106353 = t213 * t29636;
    let t106360 = t6048 * t836 * t231;
    let t106365 = t6071 * t836 * t231;
    let t106382 = 0.8673628188205199462e0_f64 * t7070 * t7071 * t7048 * t6071 + 0.8673628188205199462e0_f64 * t106172 * t27354 + 0.19274729307122665471e-1_f64 * t93286 - 0.68540937416128198416e-1_f64 * t99366 + 0.17347256376410398924e1_f64 * t25383 * t29611 - 0.65854491829355115987e0_f64 * t106353 * t887 - t99375 - 0.17347256376410398924e1_f64 * t27353 * t27357 * t62628 + 0.26020884564615598386e1_f64 * t93349 * t25392 * t106360 - 0.8673628188205199462e0_f64 * t25391 * t25392 * t106365 - 0.4336814094102599731e0_f64 * t1956 * t1957 * t233 * t106111 + 0.8673628188205199462e0_f64 * t106275 * t7073 - 0.4336814094102599731e0_f64 * t29698 * t7083 + 0.3427046870806409921e-2_f64 * t99381 + 0.34694512752820797848e1_f64 * t25391 * t27357 * t1579 * t14587;
    t106382
}
