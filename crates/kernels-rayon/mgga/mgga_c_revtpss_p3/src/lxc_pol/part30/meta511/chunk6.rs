//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1898/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1898(t233: f64, t27265: f64, t1957: f64, t1956: f64, t27183: f64, t27187: f64, t27189: f64, t27192: f64, t27196: f64, t27199: f64, t27203: f64, t27207: f64, t27214: f64, t27217: f64, t4487: f64, t4534: f64, t7053: f64, t7067: f64, t7070: f64, t7073: f64, t7779: f64, t887: f64) -> (f64, f64, f64) {
    let t27266 = t233 * t27265;
    let t27267 = t1957 * t27266;
    let t27272 = -0.65854491829355115987e0_f64 * t7053 * t4534 + 0.8673628188205199462e0_f64 * t7070 * t27183 + 0.12851425765524037203e-1_f64 * t27187 - 0.65854491829355115987e0_f64 * t27189 * t887 - 0.72280234901709995518e-2_f64 * t27192 - 0.54878743191129263322e-2_f64 * t27196 + 0.8673628188205199462e0_f64 * t27199 * t7073 + 0.9757440539382783019e-2_f64 * t27203 + 0.4336814094102599731e0_f64 * t7070 * t27207 - 0.4336814094102599731e0_f64 * t7067 * t7779 + 0.72280234901709995518e-2_f64 * t27214 - 0.12851425765524037203e-1_f64 * t27217 - 0.4336814094102599731e0_f64 * t1956 * t27267 + 0.13170898365871023197e1_f64 * t7053 * t4487;
    (t27266, t27267, t27272)
}
