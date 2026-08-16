//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3291/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3291<F: Float>(t14239: F, t22336: F, t13790: F, t6843: F, t10022: F, t2782: F, t22252: F, t4003: F, t46463: F, t48004: F, t48005: F, t48009: F, t5735: F, t5745: F, t74949: F, t74979: F, t74985: F, t74990: F) -> F {
    let t86411 = t14239 * t22336;
    let t86413 = t13790 * t6843;
    let t86415 = t2782 * t10022 * t86413;
    let t86422 = F::cast_from(0.11708928647259339623e0_f64) * t74949 + F::cast_from(0.39512695097613069591e1_f64) * t5745 * t5735 * t4003 * t22252 - F::cast_from(0.29272321618148349057e-1_f64) * t86411 - F::cast_from(0.32927245914677557992e-1_f64) * t86415 - F::cast_from(0.30356481678079769392e-1_f64) * t46463 - t48004 + F::cast_from(0.78059524315062264151e-2_f64) * t48005 - t48009 + F::cast_from(0.32927245914677557992e-1_f64) * t74979 + F::cast_from(0.16463622957338778996e-1_f64) * t74985 + F::cast_from(0.19514881078765566037e-2_f64) * t74990;
    t86422
}
