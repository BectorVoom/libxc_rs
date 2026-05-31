//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1448/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1448<F: Float>(t1134: F, t11461: F, t11466: F, t22746: F, t22750: F, t26416: F, t26433: F, t26437: F, t26534: F, t2889: F, t2893: F, t31480: F, t31496: F, t31501: F, t31504: F, t31539: F, t31540: F, t3788: F, t4562: F, t518: F, t9632: F, t9645: F) -> F {
    let t31575 = F::cast_from(2800.0_f64) * t26416 * t31501 - F::cast_from(11200.0_f64) / F::cast_from(3.0_f64) * t26437 * t31504 - F::cast_from(256.0_f64) / F::cast_from(9.0_f64) * t9632 * t31480 + F::cast_from(400.0_f64) / F::cast_from(3.0_f64) * t26534 * t31501 - F::cast_from(800.0_f64) / F::cast_from(3.0_f64) * t26433 * t31504 - F::cast_from(400.0_f64) / F::cast_from(3.0_f64) * t26534 * t31504 + F::cast_from(400.0_f64) / F::cast_from(3.0_f64) * t26534 * t31496 + F::cast_from(320.0_f64) * t22746 * t31539 * t9645 - F::cast_from(448.0_f64) * t22750 * t31540 - F::cast_from(36.0_f64) * t1134 * t4562 * t2889 + F::cast_from(42.0_f64) * t518 * t11461 * t2893 - F::cast_from(8.0_f64) * t3788 * t11466;
    t31575
}
