//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 749/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk749(t2228: f64, t838: f64, t326: f64, t8264: f64, t14516: f64, t7288: f64, t2227: f64, t36: f64, t2123: f64, t698: f64, t664: f64, t305: f64, t71835: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t71882 = t838 * t2228;
    let t71887 = t326 * t8264;
    let t71892 = t14516 * t7288;
    let t71903 = t2227 * t36;
    let t71910 = t698 * t2123;
    let t71916 = t2227 * t664;
    let t71940 = t305 * t71835;
    (t71882, t71887, t71892, t71903, t71910, t71916, t71940)
}
