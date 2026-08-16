//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1439/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1439(t2880: f64, t4574: f64, t2876: f64, t4576: f64, t3788: f64, t11474: f64, t11570: f64, t11578: f64, t22662: f64, t26421: f64, t26552: f64, t26564: f64, t26579: f64, t2869: f64, t2877: f64, t2889: f64, t2894: f64, t30764: f64, t3747: f64, t4525: f64, t4541: f64, t4545: f64, t7764: f64, t9493: f64, t9762: f64, t9769: f64) -> f64 {
    let t31279 = t4574 * t2880;
    let t31285 = t4576 * t2876;
    let t31294 = t3788 * t2880;
    let t31303 = -1936.0_f64 / 243.0_f64 * t3747 * t30764 - 320.0_f64 / 3.0_f64 * t26421 * t11474 * t9493 + 6.0_f64 * t31279 * t2877 - 1440.0_f64 * t26579 * t4576 * t2869 - 4032.0_f64 * t26564 * t31285 - 96.0_f64 * t26552 * t31285 + 6.0_f64 * t22662 * t4525 - 2.0_f64 * t7764 * t4545 + 48.0_f64 * t31294 * t11578 - 2.0_f64 * t11570 * t2894 + t9762 * t4541 - 360.0_f64 * t9769 * t4576 * t2889;
    t31303
}
