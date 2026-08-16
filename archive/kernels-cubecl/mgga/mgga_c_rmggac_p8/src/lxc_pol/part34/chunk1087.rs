//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1087/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1087<F: Float>(t72147: F, t70582: F, t2211: F, t41122: F, t884: F, t40940: F, t70556: F, t70573: F, t70577: F, t76517: F, t76539: F, t76542: F, t78567: F, t78571: F, t78572: F, t78574: F, t78575: F, t78576: F, t78577: F) -> F {
    let t78578 = F::cast_from(0.36366215538993788972e-1_f64) * t72147;
    let t78582 = F::cast_from(0.86737941314158990619e-4_f64) * t70582;
    let t78585 = F::cast_from(0.11974241701863808564e0_f64) * t884 * t2211 * t41122;
    let t78588 = F::cast_from(0.11974241701863808564e0_f64) * t884 * t2211 * t40940;
    let t78589 = t78567 + t78571 + t78572 - F::cast_from(0.16566831523319392755e-1_f64) * t76517 + t78574 - t78575 - t78576 + t78577 - t78578 + F::cast_from(0.40878380883436523436e-5_f64) * t70556 + F::cast_from(0.17347588262831798123e-4_f64) * t70573 + F::cast_from(0.17347588262831798123e-4_f64) * t70577 + t78582 + t76539 - t76542 - t78585 - t78588;
    t78589
}
