//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1451/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1451<F: Float>(t31651: F, t9656: F, t4861: F, t9634: F, t3746: F, t5204: F, t3738: F, t3785: F, t11343: F, t11349: F, t11354: F, t22754: F, t26865: F, t31051: F, t31652: F, t9632: F, t9636: F, t9639: F, t9663: F, t9670: F, sigma0: F) -> F {
    let t31656 = t31651 * t9656;
    let t31659 = t4861 * sigma0;
    let t31660 = t9634 * t31659;
    let t31663 = t5204 * t3746;
    let t31670 = t3785 * t3738;
    let t31685 = -F::cast_from(704.0_f64) / F::cast_from(27.0_f64) * t9639 * t31656 - F::cast_from(6400.0_f64) / F::cast_from(81.0_f64) * t9639 * t31660 + F::cast_from(1600.0_f64) / F::cast_from(81.0_f64) * t31663 * t9636 - F::cast_from(6400.0_f64) / F::cast_from(243.0_f64) * t9663 * t31660 - F::cast_from(6400.0_f64) / F::cast_from(243.0_f64) * t9670 * t31660 - F::cast_from(256.0_f64) / F::cast_from(81.0_f64) * t31670 * t11343 + F::cast_from(1600.0_f64) / F::cast_from(81.0_f64) * t26865 * t11349 - F::cast_from(128.0_f64) / F::cast_from(27.0_f64) * t26865 * t11354 - F::cast_from(6400.0_f64) / F::cast_from(81.0_f64) * t9632 * t31660 + F::cast_from(704.0_f64) / F::cast_from(81.0_f64) * t9663 * t31652 - F::cast_from(704.0_f64) / F::cast_from(81.0_f64) * t9670 * t31656 - F::cast_from(64.0_f64) / F::cast_from(3.0_f64) * t22754 * t31051;
    t31685
}
