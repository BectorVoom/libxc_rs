//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2131/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2131(t25082: f64, t49582: f64, t8717: f64, t2014: f64, t25089: f64, t28172: f64, t27154: f64, t95088: f64, t26089: f64, t5542: f64, t13425: f64, t13537: f64, t1843: f64, t2007: f64, t25096: f64, t28025: f64, t4246: f64, t4293: f64, t6985: f64, t7221: f64, t98426: f64, t98428: f64, t98430: f64, t98432: f64, t98439: f64, t98440: f64, t98442: f64, t98449: f64, t98452: f64, t98455: f64) -> f64 {
    let t98458 = 3.0_f64 * t25082 * t8717 * t49582;
    let t98461 = 3.0_f64 * t2014 * t28172 * t25089;
    let t98463 = 6.0_f64 * t95088 * t27154;
    let t98467 = t2014 * t26089 * t5542;
    let t98468 = -t13425 * t2007 - 2.0_f64 * t13537 * t6985 - 2.0_f64 * t1843 * t25096 - 4.0_f64 * t28025 * t4293 - 2.0_f64 * t4246 * t7221 - t98426 - t98428 - t98430 - t98432 - t98439 + t98440 - t98442 + t98449 - t98452 - t98455 - t98458 + t98461 - t98463 - t98467;
    t98468
}
