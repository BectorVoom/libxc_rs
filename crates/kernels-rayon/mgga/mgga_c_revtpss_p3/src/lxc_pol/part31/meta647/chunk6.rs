//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2130/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2130(t106430: f64, t25431: f64, t106404: f64, t106407: f64, t106410: f64, t106423: f64, t106431: f64, t1579: f64, t18313: f64, t18663: f64, t1959: f64, t25317: f64, t25391: f64, t25392: f64, t27265: f64, t29636: f64, t6048: f64, t7048: f64, t7053: f64, t7070: f64, t7071: f64, t886: f64, t93334: f64, t93335: f64, t93339: f64, t99456: f64) -> f64 {
    let t106433 = t25431 * t106430;
    let t106441 = -0.4336814094102599731e0_f64 * t106404 * t1959 - 0.9757440539382783019e-2_f64 * t106407 - 0.17347256376410398924e1_f64 * t25391 * t25392 * t106410 - 0.26020884564615598386e1_f64 * t7070 * t25317 * t7048 * t6048 + 0.8673628188205199462e0_f64 * t7070 * t7071 * t29636 * t886 + 0.10975748638225852664e-1_f64 * t106423 + 0.17347256376410398924e1_f64 * t7070 * t7071 * t27265 * t1579 - t93334 - 0.17135234354032049604e-1_f64 * t93335 + 0.12851425765524037203e-1_f64 * t106431 - 0.72280234901709995518e-2_f64 * t106433 - 0.34270468708064099208e-1_f64 * t93339 + 0.26341796731742046394e1_f64 * t7053 * t18313 - 0.39512695097613069591e1_f64 * t7053 * t18663 - 0.19274729307122665472e-1_f64 * t99456;
    t106441
}
