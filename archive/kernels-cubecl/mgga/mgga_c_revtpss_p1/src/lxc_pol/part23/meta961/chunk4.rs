//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3249/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3249<F: Float>(t22453: F, t49471: F, t47474: F, t47478: F, t47487: F, t47495: F, t47497: F, t47845: F, t47858: F, t47860: F, t47863: F, t73641: F, t73647: F, t73652: F, t73656: F, t73662: F) -> F {
    let t85484 = t49471 * t22453;
    let t85498 = t47845 + F::cast_from(0.58544643236296698112e-1_f64) * t85484 + F::cast_from(0.19514881078765566037e-2_f64) * t73641 + F::cast_from(0.30356481678079769392e-1_f64) * t47474 - F::cast_from(0.30356481678079769392e-1_f64) * t47478 + F::cast_from(0.46263278077393568556e-2_f64) * t47487 - F::cast_from(0.29272321618148349057e-1_f64) * t73647 - t47858 - F::cast_from(0.78059524315062264152e-1_f64) * t47860 + F::cast_from(0.32927245914677557992e-1_f64) * t73652 + F::cast_from(0.91069445034239308177e-1_f64) * t47863 + F::cast_from(0.34697458558045176418e-2_f64) * t73656 - F::cast_from(0.26019841438354088051e-2_f64) * t47495 + F::cast_from(0.17073386770573548589e-1_f64) * t47497 + F::cast_from(0.39029762157531132076e-1_f64) * t73662;
    t85498
}
