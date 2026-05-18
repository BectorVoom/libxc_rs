//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 875/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk875<F: Float>(t10066: F, t36772: F, t40759: F, t8626: F, t623: F, t8629: F, t8632: F, t1734: F, t352: F, t3928: F, t6418: F, t645: F) -> (F, F, F, F, F) {
    let t44700 = t36772 * t10066;
    let t44702 = t40759 * t8626;
    let t44705 = t623 * t8629 * t8632;
    let t44713 = t1734 * t352;
    let t44724 = t3928 * t645 * t6418;
    (t44700, t44702, t44705, t44713, t44724)
}
