//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1449/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1449(t2874: f64, t4530: f64, t1117: f64, t1134: f64, t11421: f64, t11430: f64, t11437: f64, t11454: f64, t11465: f64, t2869: f64, t2876: f64, t2889: f64, t2893: f64, t2903: f64, t4521: f64, t4550: f64, t4553: f64, t4556: f64, t4559: f64, t4568: f64, t510: f64, t518: f64, t7817: f64, t9747: f64) -> f64 {
    let t31605 = t2874 * t4530;
    let t31612 = -4.0_f64 * t1117 * t11465 * t2893 + 12.0_f64 * t1117 * t4550 * t2889 - 4.0_f64 * t1117 * t4568 * t2889 - 36.0_f64 * t1134 * t11454 * t2893 - 336.0_f64 * t518 * t11421 * t2893 + 6.0_f64 * t510 * t11430 * t2893 - 24.0_f64 * t510 * t11437 * t2893 + 1260.0_f64 * t2903 * t4556 * t2869 + 1260.0_f64 * t2903 * t31605 * t2876 + 30.0_f64 * t2903 * t4559 * t2889 + 120.0_f64 * t7817 * t4553 * t2889 - 8.0_f64 * t9747 * t4521;
    t31612
}
