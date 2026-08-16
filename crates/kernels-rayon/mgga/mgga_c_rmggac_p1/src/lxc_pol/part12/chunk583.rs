//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 583/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk583(t262: f64, t7634: f64, t7633: f64, t7596: f64, t2100: f64, t22: f64, t3839: f64, t7614: f64, t7617: f64, t2103: f64, t3826: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7635 = t262 * t7634;
    let t7636 = t7633 * t7635;
    let t7638 = t262 * t7596;
    let t7639 = t2100 * t7638;
    let t7640 = 0.18183107769496894486e-1_f64 * t7639;
    let t7641 = t3839 * t22;
    let t7642 = t262 * t7614;
    let t7643 = t7641 * t7642;
    let t7645 = t262 * t7617;
    let t7646 = t2103 * t7645;
    let t7647 = 0.24244143692662525982e-1_f64 * t7646;
    let t7648 = t3826 * t22;
    (t7635, t7636, t7638, t7639, t7640, t7641, t7642, t7643, t7645, t7646, t7647, t7648)
}
