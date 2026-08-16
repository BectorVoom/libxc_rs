//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1449/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1449<F: Float>(t2874: F, t4530: F, t1117: F, t1134: F, t11421: F, t11430: F, t11437: F, t11454: F, t11465: F, t2869: F, t2876: F, t2889: F, t2893: F, t2903: F, t4521: F, t4550: F, t4553: F, t4556: F, t4559: F, t4568: F, t510: F, t518: F, t7817: F, t9747: F) -> F {
    let t31605 = t2874 * t4530;
    let t31612 = -F::cast_from(4.0_f64) * t1117 * t11465 * t2893 + F::cast_from(12.0_f64) * t1117 * t4550 * t2889 - F::cast_from(4.0_f64) * t1117 * t4568 * t2889 - F::cast_from(36.0_f64) * t1134 * t11454 * t2893 - F::cast_from(336.0_f64) * t518 * t11421 * t2893 + F::cast_from(6.0_f64) * t510 * t11430 * t2893 - F::cast_from(24.0_f64) * t510 * t11437 * t2893 + F::cast_from(1260.0_f64) * t2903 * t4556 * t2869 + F::cast_from(1260.0_f64) * t2903 * t31605 * t2876 + F::cast_from(30.0_f64) * t2903 * t4559 * t2889 + F::cast_from(120.0_f64) * t7817 * t4553 * t2889 - F::cast_from(8.0_f64) * t9747 * t4521;
    t31612
}
