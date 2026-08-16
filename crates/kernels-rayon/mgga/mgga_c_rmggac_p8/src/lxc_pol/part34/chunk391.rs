//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 391/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk391(t7596: f64, t851: f64, t7617: f64, t854: f64, t305: f64, t830: f64, t262: f64, t2100: f64, t2103: f64, t2115: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7620 = t851 * t7596;
    let t7625 = t854 * t7617;
    let t7627 = t305 * t830;
    let t7628 = 0.48783947674259960818e-1_f64 * t7627;
    let t7638 = t262 * t7596;
    let t7639 = t2100 * t7638;
    let t7640 = 0.18183107769496894486e-1_f64 * t7639;
    let t7645 = t262 * t7617;
    let t7646 = t2103 * t7645;
    let t7647 = 0.24244143692662525982e-1_f64 * t7646;
    let t7651 = t2115 * t7638;
    let t7652 = 0.4838420607177634088e-3_f64 * t7651;
    (t7620, t7625, t7627, t7628, t7638, t7639, t7640, t7645, t7646, t7647, t7651, t7652)
}
