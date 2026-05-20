//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1630/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1630<F: Float>(t14586: F, t1559: F, t18677: F, t4504: F, t4514: F, t51578: F, t51635: F, t6017: F, t62909: F, t62920: F, t62922: F, t62952: F, t62983: F, t62999: F, t77159: F, t77225: F, t820: F) -> F {
    let t87869 = F::cast_from(0.7805952431506226415e-2_f64) * t62909 + F::cast_from(0.39029762157531132075e-2_f64) * t62920 - F::cast_from(0.87805989105806821314e-1_f64) * t62922 + F::cast_from(0.15805078039045227836e2_f64) * t4504 * t77159 * t14586 - F::cast_from(0.44178176337912614788e-3_f64) * t51578 + F::cast_from(0.78059524315062264152e-1_f64) * t62952 - F::cast_from(0.26341796731742046395e1_f64) * t820 * t77225 * t1559 + F::cast_from(0.1561190486301245283e0_f64) * t62983 + F::cast_from(0.18505311230957427423e-1_f64) * t51635 - F::cast_from(0.39512695097613069592e1_f64) * t4514 * t18677 * t6017 - F::cast_from(0.69394917116090352835e-2_f64) * t62999;
    t87869
}
