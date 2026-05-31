//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1205/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1205<F: Float>(t2856: F, t528: F, t530: F, t22703: F, t2867: F, t7805: F, t1143: F, t9557: F, t1166: F, t9526: F, t509: F, t515: F) -> (F, F, F, F, F, F) {
    let t22714 = F::cast_from(1.0_f64) / t530 / t2856 / t528 / F::cast_from(2.0_f64);
    let t22723 = F::cast_from(1.0_f64) / t22703;
    let t22746 = t2867 * t7805;
    let t22750 = t1143 * t9557;
    let t22754 = t1166 * t9526;
    let t22809 = t515 * t509;
    (t22714, t22723, t22746, t22750, t22754, t22809)
}
