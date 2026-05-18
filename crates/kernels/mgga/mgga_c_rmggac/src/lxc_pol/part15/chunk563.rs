//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 563/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk563<F: Float>(t265: F, t333: F, t797: F, t7596: F, t851: F, t854: F, t305: F, t830: F, t22: F, t3851: F, t262: F, t2100: F) -> (F, F, F, F, F, F, F, F) {
    let t7617 = t265 * t333;
    let t7618 = t797 * t7617;
    let t7620 = t851 * t7596;
    let t7625 = t854 * t7617;
    let t7627 = t305 * t830;
    let t7628 = F::new(0.48783947674259960818e-1) * t7627;
    let t7633 = t3851 * t22;
    let t7638 = t262 * t7596;
    let t7639 = t2100 * t7638;
    (t7617, t7618, t7620, t7625, t7628, t7633, t7638, t7639)
}
