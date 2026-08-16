//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1138/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1138(t25413: f64, t25431: f64, t1959: f64, t25362: f64, t25364: f64, t25366: f64, t25368: f64, t25371: f64, t25379: f64, t25383: f64, t25388: f64, t25391: f64, t25395: f64, t25400: f64, t25406: f64, t25407: f64, t25414: f64, t25419: f64, t25424: f64, t25426: f64, t2829: f64, t7053: f64, t7070: f64, t7073: f64, t7079: f64) -> f64 {
    let t25432 = t25431 * t25413;
    let t25434 = -t25362 - t25364 - 0.25702851531048074406e-1_f64 * t25366 - 0.25702851531048074406e-1_f64 * t25368 + t25371 - 0.28912093960683998208e-1_f64 * t25379 - 0.65854491829355115987e0_f64 * t7053 * t2829 + 0.17347256376410398924e1_f64 * t25383 * t7073 + 0.51405703062096148812e-1_f64 * t25388 - 0.17347256376410398924e1_f64 * t25391 * t25395 - 0.19514881078765566038e-1_f64 * t25400 - t25406 - 0.4336814094102599731e0_f64 * t25407 * t1959 + 0.25702851531048074406e-1_f64 * t25414 - 0.8673628188205199462e0_f64 * t7070 * t25419 + t25424 + 0.4336814094102599731e0_f64 * t7070 * t25426 + 0.8673628188205199462e0_f64 * t25383 * t7079 - 0.14456046980341999104e-1_f64 * t25432;
    t25434
}
