//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1943/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1943(t1959: f64, t25333: f64, t25337: f64, t25362: f64, t25364: f64, t25371: f64, t25391: f64, t25406: f64, t25424: f64, t27199: f64, t27280: f64, t27325: f64, t27335: f64, t27338: f64, t27342: f64, t27344: f64, t29675: f64, t29683: f64, t29691: f64, t29695: f64, t29698: f64, t7070: f64, t7775: f64) -> f64 {
    let t29703 = 0.4336814094102599731e0_f64 * t7070 * t29675 + 0.8673628188205199462e0_f64 * t27199 * t7775 + t25333 - 0.25702851531048074406e-1_f64 * t27280 - t25337 - t25362 - t25364 + t25371 - 0.17347256376410398924e1_f64 * t25391 * t29683 - 0.19514881078765566038e-1_f64 * t27325 - t25406 + 0.10975748638225852664e-1_f64 * t27335 + 0.14456046980341999104e-1_f64 * t27338 + 0.4336814094102599731e0_f64 * t7070 * t29691 - 0.8673628188205199462e0_f64 * t7070 * t29695 + t25424 - 0.4336814094102599731e0_f64 * t29698 * t1959 - 0.28912093960683998208e-1_f64 * t27342 + 0.51405703062096148812e-1_f64 * t27344;
    t29703
}
