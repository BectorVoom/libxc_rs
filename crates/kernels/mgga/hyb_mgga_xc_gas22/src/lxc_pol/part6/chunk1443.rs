//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1443/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1443<F: Float>(t1145: F, t2893: F, t4540: F, t2889: F, t4544: F, t11549: F, t26927: F, t2869: F, t2875: F, t2881: F, t2922: F, t2927: F, t31363: F, t31367: F, t4530: F, t4577: F, t7721: F, t7739: F, t7769: F, t7775: F, t9782: F) -> F {
    let t31382 = t1145 * t4540 * t2893;
    let t31386 = t1145 * t4544 * t2889;
    let t31390 = t1145 * t4544 * t2893;
    let t31405 = -F::cast_from(90.0_f64) * t7721 * t1145 * t4544 * t2869 + F::cast_from(60.0_f64) * t7775 * t1145 * t4530 * t2889 - F::cast_from(4.0_f64) * t9782 * t11549 - F::cast_from(4.0_f64) * t26927 * t4577 + F::cast_from(21.0_f64) * t2875 * t31390 + F::cast_from(3.0_f64) * t2881 * t31390 - F::cast_from(18.0_f64) * t2922 * t31382 - F::cast_from(18.0_f64) * t2922 * t31386 - F::cast_from(2.0_f64) * t2927 * t31382 - F::cast_from(2.0_f64) * t2927 * t31386 + F::cast_from(6.0_f64) * t7739 * t31363 - F::cast_from(12.0_f64) * t7769 * t31367;
    t31405
}
