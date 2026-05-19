//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 583/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk583<F: Float>(t262: F, t7634: F, t7633: F, t7596: F, t2100: F, t22: F, t3839: F, t7614: F, t7617: F, t2103: F, t3826: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t7635 = t262 * t7634;
    let t7636 = t7633 * t7635;
    let t7638 = t262 * t7596;
    let t7639 = t2100 * t7638;
    let t7640 = F::cast_from(0.18183107769496894486e-1_f64) * t7639;
    let t7641 = t3839 * t22;
    let t7642 = t262 * t7614;
    let t7643 = t7641 * t7642;
    let t7645 = t262 * t7617;
    let t7646 = t2103 * t7645;
    let t7647 = F::cast_from(0.24244143692662525982e-1_f64) * t7646;
    let t7648 = t3826 * t22;
    (t7635, t7636, t7638, t7639, t7640, t7641, t7642, t7643, t7645, t7646, t7647, t7648)
}
