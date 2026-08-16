//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 746/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk746(t1173: f64, t674: f64, t7942: f64, t34884: f64, t7733: f64, t2185: f64, t7716: f64, t1997: f64, t1004: f64, t107: f64, t490: f64, t7288: f64, t7494: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t35146 = t7942 * t1173 * t674;
    let t35149 = t34884 * t7733;
    let t35151 = t7716 * t2185;
    let t35152 = t35151 * t1997;
    let t35154 = t1004 * t107;
    let t35155 = t490 * t35154;
    let t35184 = t7494 * t7288;
    (t35146, t35149, t35151, t35152, t35155, t35184)
}
