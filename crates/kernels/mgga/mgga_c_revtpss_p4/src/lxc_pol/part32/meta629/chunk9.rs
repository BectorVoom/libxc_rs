//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2028/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2028<F: Float>(t6049: F, t689: F, t7384: F, t1580: F, t28447: F, t110502: F, t25387: F, t103449: F, t103462: F, t103463: F, t110444: F, t1956: F, t1957: F, t231: F, t233: F, t25317: F, t28394: F, t28442: F, t30341: F, t30379: F, t4534: F, t7070: F, t7076: F, t836: F, t886: F, t95888: F, t95891: F, t95893: F, t99191: F) -> F {
    let t110584 = t689 * t7384 * t6049;
    let t110591 = t689 * t28447 * t1580;
    let t110600 = t25387 * t110502;
    let t110607 = -F::cast_from(0.52041769129231196772e1_f64) * t7070 * t25317 * t30341 * t886 - F::cast_from(0.10975748638225852664e-1_f64) * t110584 - F::cast_from(0.4336814094102599731e0_f64) * t1956 * t1957 * t233 * t110444 + F::cast_from(0.10975748638225852664e-1_f64) * t110591 + F::cast_from(0.4336814094102599731e0_f64) * t7070 * t7076 * t30379 * t836 * t231 - F::cast_from(0.17347256376410398924e1_f64) * t99191 * t28442 + F::cast_from(0.51405703062096148813e-1_f64) * t110600 - F::cast_from(0.26019841438354088051e-1_f64) * t103449 + F::cast_from(0.17135234354032049604e-1_f64) * t95888 + t95891 - t103462 + F::cast_from(0.3427046870806409921e-2_f64) * t103463 - t95893 - F::cast_from(0.13170898365871023197e1_f64) * t28394 * t4534;
    t110607
}
