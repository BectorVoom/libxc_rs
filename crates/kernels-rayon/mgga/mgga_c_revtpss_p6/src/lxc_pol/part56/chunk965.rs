//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 965/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk965(t1203: f64, t8931: f64, t33462: f64, t1214: f64, t1243: f64, t1248: f64, t1287: f64, t1294: f64, t2150: f64, t33398: f64, t33401: f64, t33405: f64, t33408: f64, t33414: f64, t33417: f64, t33423: f64, t33425: f64, t33428: f64, t33433: f64, t33436: f64, t33441: f64, t33446: f64, t33449: f64, t33456: f64, t33461: f64, t473: f64, t7666: f64, t8926: f64, t8932: f64) -> (f64, f64) {
    let t33463 = t8931 * t1203;
    let t33464 = t33462 * t33463;
    let t33467 = 0.56468933516960933998e-3_f64 * t33398 * t33401 - 0.56468933516960933998e-3_f64 * t33405 * t33408 + 0.28234466758480466999e-3_f64 * t33414 * t33417 + t33423 - 0.18822977838986977999e-3_f64 * t33425 * t33428 - 0.28234466758480466999e-3_f64 * t8926 * t33433 - 0.17347256376410398924e1_f64 * t33436 * t2150 * t473 * t1203 + 0.17347256376410398924e1_f64 * t33441 * t2150 * t473 * t1214 - 0.17347256376410398924e1_f64 * t33446 * t7666 + 0.17347256376410398924e1_f64 * t8932 * t33449 * t473 * t1294 - 0.8673628188205199462e0_f64 * t33456 * t1243 * t1248 * t1287 + 0.17135921299530705785e1_f64 * t33461 * t33464;
    (t33464, t33467)
}
