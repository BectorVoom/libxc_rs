//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 769/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk769(t12021: f64, t28223: f64, t22933: f64, t6439: f64, t6889: f64, t1985: f64, t1375: f64, t1843: f64, t20060: f64, t2016: f64, t22924: f64, t22926: f64, t26366: f64, t26475: f64, t27067: f64, t28193: f64, t28196: f64, t28201: f64, t28207: f64, t28211: f64, t28214: f64, t28220: f64, t5321: f64, t6440: f64, t6958: f64, t7729: f64, t7750: f64) -> (f64, f64, f64, f64) {
    let t28224 = t12021 * t28223;
    let t28232 = t22933 * t6439;
    let t28233 = t6889 * t28232;
    let t28234 = t1985 * t28233;
    let t28236 = 0.49348022005446793095e-1_f64 * t28193 - 0.16449340668482264365e-1_f64 * t28196 + 0.82246703342411321825e-2_f64 * t28201 - 2.0_f64 * t26366 * t1843 - 0.82246703342411321825e-2_f64 * t28207 - 0.16449340668482264365e-1_f64 * t28211 - 0.3289868133696452873e-1_f64 * t28214 - t27067 - 2.0_f64 * t5321 * t7750 - 0.82246703342411321824e-2_f64 * t26475 + 4.0_f64 * t1375 * t28220 - 6.0_f64 * t1375 * t28224 + 4.0_f64 * t5321 * t7729 + 2.0_f64 * t6958 * t6440 - t20060 * t2016 + 0.16449340668482264365e-1_f64 * t28234 + t22924 + t22926;
    (t28224, t28232, t28234, t28236)
}
