//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2028/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2028(t6049: f64, t689: f64, t7384: f64, t1580: f64, t28447: f64, t110502: f64, t25387: f64, t103449: f64, t103462: f64, t103463: f64, t110444: f64, t1956: f64, t1957: f64, t231: f64, t233: f64, t25317: f64, t28394: f64, t28442: f64, t30341: f64, t30379: f64, t4534: f64, t7070: f64, t7076: f64, t836: f64, t886: f64, t95888: f64, t95891: f64, t95893: f64, t99191: f64) -> f64 {
    let t110584 = t689 * t7384 * t6049;
    let t110591 = t689 * t28447 * t1580;
    let t110600 = t25387 * t110502;
    let t110607 = -0.52041769129231196772e1_f64 * t7070 * t25317 * t30341 * t886 - 0.10975748638225852664e-1_f64 * t110584 - 0.4336814094102599731e0_f64 * t1956 * t1957 * t233 * t110444 + 0.10975748638225852664e-1_f64 * t110591 + 0.4336814094102599731e0_f64 * t7070 * t7076 * t30379 * t836 * t231 - 0.17347256376410398924e1_f64 * t99191 * t28442 + 0.51405703062096148813e-1_f64 * t110600 - 0.26019841438354088051e-1_f64 * t103449 + 0.17135234354032049604e-1_f64 * t95888 + t95891 - t103462 + 0.3427046870806409921e-2_f64 * t103463 - t95893 - 0.13170898365871023197e1_f64 * t28394 * t4534;
    t110607
}
