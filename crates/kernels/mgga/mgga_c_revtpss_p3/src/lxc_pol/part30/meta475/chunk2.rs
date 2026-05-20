//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1796/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1796<F: Float>(t25413: F, t25431: F, t1959: F, t25362: F, t25364: F, t25366: F, t25368: F, t25371: F, t25379: F, t25383: F, t25388: F, t25391: F, t25395: F, t25400: F, t25406: F, t25407: F, t25414: F, t25419: F, t25424: F, t25426: F, t2829: F, t7053: F, t7070: F, t7073: F, t7079: F) -> (F, F) {
    let t25432 = t25431 * t25413;
    let t25434 = -t25362 - t25364 - F::cast_from(0.25702851531048074406e-1_f64) * t25366 - F::cast_from(0.25702851531048074406e-1_f64) * t25368 + t25371 - F::cast_from(0.28912093960683998208e-1_f64) * t25379 - F::cast_from(0.65854491829355115987e0_f64) * t7053 * t2829 + F::cast_from(0.17347256376410398924e1_f64) * t25383 * t7073 + F::cast_from(0.51405703062096148812e-1_f64) * t25388 - F::cast_from(0.17347256376410398924e1_f64) * t25391 * t25395 - F::cast_from(0.19514881078765566038e-1_f64) * t25400 - t25406 - F::cast_from(0.4336814094102599731e0_f64) * t25407 * t1959 + F::cast_from(0.25702851531048074406e-1_f64) * t25414 - F::cast_from(0.8673628188205199462e0_f64) * t7070 * t25419 + t25424 + F::cast_from(0.4336814094102599731e0_f64) * t7070 * t25426 + F::cast_from(0.8673628188205199462e0_f64) * t25383 * t7079 - F::cast_from(0.14456046980341999104e-1_f64) * t25432;
    (t25432, t25434)
}
