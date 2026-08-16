//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1036/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1036(t2056: f64, t32392: f64, t7002: f64, t94: f64, t7367: f64, t8634: f64, t2322: f64, t8641: f64, t4254: f64, t25805: f64, t28025: f64, t32386: f64, t32388: f64, t32389: f64, t671: f64, t6985: f64, t7007: f64, t7359: f64, t7374: f64) -> (f64, f64) {
    let t32393 = t32392 * t2056;
    let t32394 = t94 * t7002;
    let t32395 = t32394 * t2056;
    let t32396 = t8634 * t7367;
    let t32397 = t2322 * t8641;
    let t32398 = t4254 * t8641;
    let t32399 = -t2056 * t25805 - t2056 * t28025 - t32389 * t671 - t6985 * t7367 - t6985 * t7374 - t7007 * t7359 - t32386 - t32388 - t32393 - t32395 - t32396 - t32397 - t32398;
    (t32394, t32399)
}
