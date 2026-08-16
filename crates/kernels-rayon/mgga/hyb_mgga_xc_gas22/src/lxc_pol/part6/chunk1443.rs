//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1443/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1443(t1145: f64, t2893: f64, t4540: f64, t2889: f64, t4544: f64, t11549: f64, t26927: f64, t2869: f64, t2875: f64, t2881: f64, t2922: f64, t2927: f64, t31363: f64, t31367: f64, t4530: f64, t4577: f64, t7721: f64, t7739: f64, t7769: f64, t7775: f64, t9782: f64) -> f64 {
    let t31382 = t1145 * t4540 * t2893;
    let t31386 = t1145 * t4544 * t2889;
    let t31390 = t1145 * t4544 * t2893;
    let t31405 = -90.0_f64 * t7721 * t1145 * t4544 * t2869 + 60.0_f64 * t7775 * t1145 * t4530 * t2889 - 4.0_f64 * t9782 * t11549 - 4.0_f64 * t26927 * t4577 + 21.0_f64 * t2875 * t31390 + 3.0_f64 * t2881 * t31390 - 18.0_f64 * t2922 * t31382 - 18.0_f64 * t2922 * t31386 - 2.0_f64 * t2927 * t31382 - 2.0_f64 * t2927 * t31386 + 6.0_f64 * t7739 * t31363 - 12.0_f64 * t7769 * t31367;
    t31405
}
