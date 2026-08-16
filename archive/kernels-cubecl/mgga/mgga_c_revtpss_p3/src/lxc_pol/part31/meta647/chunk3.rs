//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2127/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2127<F: Float>(t213: F, t29636: F, t231: F, t6048: F, t836: F, t6071: F, t106111: F, t106172: F, t106275: F, t14587: F, t1579: F, t1956: F, t1957: F, t233: F, t25383: F, t25391: F, t25392: F, t27353: F, t27354: F, t27357: F, t29611: F, t29698: F, t62628: F, t7048: F, t7070: F, t7071: F, t7073: F, t7083: F, t887: F, t93286: F, t93349: F, t99366: F, t99375: F, t99381: F) -> F {
    let t106353 = t213 * t29636;
    let t106360 = t6048 * t836 * t231;
    let t106365 = t6071 * t836 * t231;
    let t106382 = F::cast_from(0.8673628188205199462e0_f64) * t7070 * t7071 * t7048 * t6071 + F::cast_from(0.8673628188205199462e0_f64) * t106172 * t27354 + F::cast_from(0.19274729307122665471e-1_f64) * t93286 - F::cast_from(0.68540937416128198416e-1_f64) * t99366 + F::cast_from(0.17347256376410398924e1_f64) * t25383 * t29611 - F::cast_from(0.65854491829355115987e0_f64) * t106353 * t887 - t99375 - F::cast_from(0.17347256376410398924e1_f64) * t27353 * t27357 * t62628 + F::cast_from(0.26020884564615598386e1_f64) * t93349 * t25392 * t106360 - F::cast_from(0.8673628188205199462e0_f64) * t25391 * t25392 * t106365 - F::cast_from(0.4336814094102599731e0_f64) * t1956 * t1957 * t233 * t106111 + F::cast_from(0.8673628188205199462e0_f64) * t106275 * t7073 - F::cast_from(0.4336814094102599731e0_f64) * t29698 * t7083 + F::cast_from(0.3427046870806409921e-2_f64) * t99381 + F::cast_from(0.34694512752820797848e1_f64) * t25391 * t27357 * t1579 * t14587;
    t106382
}
