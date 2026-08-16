//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 777/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk777<F: Float>(t14180: F, t39277: F, t12012: F, t69045: F, t11729: F, t69041: F, t11732: F, t3046: F, t3924: F, t507: F, t12140: F, t69788: F) -> (F, F, F, F, F) {
    let t74049 = F::cast_from(0.1064114997332445985e-4_f64) * t39277 * t14180;
    let t74050 = t69045 * t12012;
    let t74052 = t69041 * t11729;
    let t74056 = t507 * t3924 * t3046 * t11732;
    let t74058 = t69788 * t12140;
    (t74049, t74050, t74052, t74056, t74058)
}
