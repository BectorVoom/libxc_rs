//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2130/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2130<F: Float>(t231: F, t25317: F, t25383: F, t25391: F, t27199: F, t27207: F, t29636: F, t29654: F, t29682: F, t29683: F, t29695: F, t7070: F, t7076: F, t836: F, t886: F, t92864: F, t92917: F, t93184: F, t93192: F, t93195: F, t99234: F, t99243: F, t99245: F, t99258: F, t99261: F) -> F {
    let t106215 = t99234 - F::cast_from(0.17347256376410398924e1_f64) * t92917 * t29683 - F::cast_from(0.17347256376410398924e1_f64) * t25391 * t92864 * t29682 + F::cast_from(0.96373646535613327357e-2_f64) * t93184 + F::cast_from(0.4336814094102599731e0_f64) * t7070 * t7076 * t29636 * t836 * t231 - F::cast_from(0.26020884564615598386e1_f64) * t7070 * t25317 * t29654 * t886 - t99243 + t99245 + F::cast_from(0.45699670022203476294e-2_f64) * t93192 + F::cast_from(0.3427046870806409921e-2_f64) * t99258 + F::cast_from(0.4818682326780666368e-3_f64) * t99261 - F::cast_from(0.11565819519348392139e-2_f64) * t93195 + F::cast_from(0.8673628188205199462e0_f64) * t27199 * t27207 - F::cast_from(0.8673628188205199462e0_f64) * t25383 * t29695;
    t106215
}
