//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 77/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk77<F: Float>(t20: F, t43: F, t40: F, t41: F, t21: F, t22: F, t26: F, t29: F) -> (F, F, F, F, F) {
    let t255 = t20 * t43;
    let t259 = F::cast_from(1.0_f64) / t41 / t40;
    let t260 = t21 * t259;
    let t261 = t260 * t22;
    let t262 = t26 * t29;
    (t255, t259, t260, t261, t262)
}
