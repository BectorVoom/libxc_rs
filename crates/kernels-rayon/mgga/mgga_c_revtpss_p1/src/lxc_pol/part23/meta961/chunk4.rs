//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3249/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3249(t22453: f64, t49471: f64, t47474: f64, t47478: f64, t47487: f64, t47495: f64, t47497: f64, t47845: f64, t47858: f64, t47860: f64, t47863: f64, t73641: f64, t73647: f64, t73652: f64, t73656: f64, t73662: f64) -> f64 {
    let t85484 = t49471 * t22453;
    let t85498 = t47845 + 0.58544643236296698112e-1_f64 * t85484 + 0.19514881078765566037e-2_f64 * t73641 + 0.30356481678079769392e-1_f64 * t47474 - 0.30356481678079769392e-1_f64 * t47478 + 0.46263278077393568556e-2_f64 * t47487 - 0.29272321618148349057e-1_f64 * t73647 - t47858 - 0.78059524315062264152e-1_f64 * t47860 + 0.32927245914677557992e-1_f64 * t73652 + 0.91069445034239308177e-1_f64 * t47863 + 0.34697458558045176418e-2_f64 * t73656 - 0.26019841438354088051e-2_f64 * t47495 + 0.17073386770573548589e-1_f64 * t47497 + 0.39029762157531132076e-1_f64 * t73662;
    t85498
}
