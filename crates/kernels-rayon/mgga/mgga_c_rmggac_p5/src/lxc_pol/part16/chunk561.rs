//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 561/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk561(t7639: f64, t22: f64, t3839: f64, t262: f64, t7617: f64, t2103: f64, t3826: f64, t2115: f64, t7638: f64, t3819: f64, t2118: f64, t344: f64, t830: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7640 = 0.18183107769496894486e-1_f64 * t7639;
    let t7641 = t3839 * t22;
    let t7645 = t262 * t7617;
    let t7646 = t2103 * t7645;
    let t7647 = 0.24244143692662525982e-1_f64 * t7646;
    let t7648 = t3826 * t22;
    let t7651 = t2115 * t7638;
    let t7652 = 0.4838420607177634088e-3_f64 * t7651;
    let t7653 = t3819 * t22;
    let t7656 = t2118 * t7645;
    let t7662 = t344 * t830;
    (t7640, t7641, t7645, t7647, t7648, t7652, t7653, t7656, t7662)
}
