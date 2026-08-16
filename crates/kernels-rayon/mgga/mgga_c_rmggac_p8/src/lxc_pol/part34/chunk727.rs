//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 727/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk727(t14148: f64, t14150: f64, t35718: f64, t240: f64, t356: f64, t4738: f64, t7351: f64, t14107: f64, t6477: f64, t14207: f64, t2604: f64, t1966: f64, t1968: f64, t68889: f64) -> (f64, f64, f64, f64, f64) {
    let t70517 = t14148 * t35718 * t14150;
    let t70524 = t14148 * t7351 * t356 * t240 * t4738;
    let t70526 = t6477 * t14107;
    let t70545 = t2604 * t14207;
    let t70548 = t1966 * t68889 * t1968;
    (t70517, t70524, t70526, t70545, t70548)
}
