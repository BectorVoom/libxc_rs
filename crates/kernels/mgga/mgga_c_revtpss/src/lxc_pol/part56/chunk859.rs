//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 859/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk859<F: Float>(t1203: F, t8931: F, t33462: F, t1214: F, t1243: F, t1248: F, t1287: F, t1294: F, t2150: F, t33398: F, t33401: F, t33405: F, t33408: F, t33414: F, t33417: F, t33423: F, t33425: F, t33428: F, t33433: F, t33436: F, t33441: F, t33446: F, t33449: F, t33456: F, t33461: F, t473: F, t7666: F, t8926: F, t8932: F) -> (F, F) {
    let t33463 = t8931 * t1203;
    let t33464 = t33462 * t33463;
    let t33467 = 0.56468933516960933998e-3 * t33398 * t33401 - 0.56468933516960933998e-3 * t33405 * t33408 + 0.28234466758480466999e-3 * t33414 * t33417 + t33423 - 0.18822977838986977999e-3 * t33425 * t33428 - 0.28234466758480466999e-3 * t8926 * t33433 - 0.17347256376410398924e1 * t33436 * t2150 * t473 * t1203 + 0.17347256376410398924e1 * t33441 * t2150 * t473 * t1214 - 0.17347256376410398924e1 * t33446 * t7666 + 0.17347256376410398924e1 * t8932 * t33449 * t473 * t1294 - 0.8673628188205199462e0 * t33456 * t1243 * t1248 * t1287 + 0.17135921299530705785e1 * t33461 * t33464;
    (t33464, t33467)
}
