//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 833/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk833(t25399: f64, t4481: f64, t1580: f64, t213: f64, t25322: f64, t25362: f64, t25364: f64, t25366: f64, t25368: f64, t25371: f64, t25379: f64, t25391: f64, t257: f64, t27199: f64, t27300: f64, t27303: f64, t27313: f64, t27317: f64, t27322: f64, t7070: f64, t7079: f64) -> f64 {
    let t27325 = t25399 * t4481;
    let t27329 = -0.26020884564615598386e1_f64 * t7070 * t27300 - t25362 + 0.65854491829355115987e0_f64 * t213 * t27303 * t257 + 0.4336814094102599731e0_f64 * t27199 * t7079 - t25364 - 0.12851425765524037203e-1_f64 * t25366 - 0.12851425765524037203e-1_f64 * t25368 + t25371 - 0.8673628188205199462e0_f64 * t25391 * t27313 + 0.8673628188205199462e0_f64 * t7070 * t27317 - 0.14456046980341999104e-1_f64 * t25379 + 0.8673628188205199462e0_f64 * t7070 * t27322 - 0.9757440539382783019e-2_f64 * t27325 - 0.65854491829355115987e0_f64 * t25322 * t1580;
    t27329
}
