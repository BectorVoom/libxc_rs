//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2130/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2130<F: Float>(t106430: F, t25431: F, t106404: F, t106407: F, t106410: F, t106423: F, t106431: F, t1579: F, t18313: F, t18663: F, t1959: F, t25317: F, t25391: F, t25392: F, t27265: F, t29636: F, t6048: F, t7048: F, t7053: F, t7070: F, t7071: F, t886: F, t93334: F, t93335: F, t93339: F, t99456: F) -> F {
    let t106433 = t25431 * t106430;
    let t106441 = -F::cast_from(0.4336814094102599731e0_f64) * t106404 * t1959 - F::cast_from(0.9757440539382783019e-2_f64) * t106407 - F::cast_from(0.17347256376410398924e1_f64) * t25391 * t25392 * t106410 - F::cast_from(0.26020884564615598386e1_f64) * t7070 * t25317 * t7048 * t6048 + F::cast_from(0.8673628188205199462e0_f64) * t7070 * t7071 * t29636 * t886 + F::cast_from(0.10975748638225852664e-1_f64) * t106423 + F::cast_from(0.17347256376410398924e1_f64) * t7070 * t7071 * t27265 * t1579 - t93334 - F::cast_from(0.17135234354032049604e-1_f64) * t93335 + F::cast_from(0.12851425765524037203e-1_f64) * t106431 - F::cast_from(0.72280234901709995518e-2_f64) * t106433 - F::cast_from(0.34270468708064099208e-1_f64) * t93339 + F::cast_from(0.26341796731742046394e1_f64) * t7053 * t18313 - F::cast_from(0.39512695097613069591e1_f64) * t7053 * t18663 - F::cast_from(0.19274729307122665472e-1_f64) * t99456;
    t106441
}
