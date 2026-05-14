//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 43/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk43<F: Float>(t127: F, t6: F, t123: F, t121: F, t21: F, t2: F) -> (F, F, F, F, F, F) {
    let t128 = t6 * t127;
    let t129 = t123 * t128;
    let t132 = 1.0 + 0.53972366148531951642e-1 * t121 * t129;
    let t133 = f64::ln(t132);
    let t135 = 1.0 + 0.193e0 * t133;
    let t136 = 1.0 / t135;
    let t138 = 1.0 / t21;
    let t139 = t2 * t138;
    (t128, t129, t132, t135, t136, t139)
}
