//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 862/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk862<F: Float>(t3351: F, t618: F, t7231: F, t875: F, t876: F, t839: F, t880: F, t236: F, t35155: F, t794: F, t16156: F, t9106: F) -> (F, F, F, F) {
    let t39238 = t3351 * t7231 * t875 * t618 * t876;
    let t39243 = t3351 * t7231 * t880 * t618 * t839;
    let t39248 = t3351 * t35155 * t236 * t618 * t794;
    let t39250 = t16156 * t9106;
    (t39238, t39243, t39248, t39250)
}
