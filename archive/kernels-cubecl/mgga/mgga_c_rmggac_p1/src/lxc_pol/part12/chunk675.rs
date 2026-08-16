//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 675/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk675<F: Float>(t338: F, t8794: F, t118: F, t1614: F, t665: F, t321: F, t8936: F, t797: F, t8884: F, t5148: F, t8621: F, t5259: F, t8649: F) -> (F, F, F, F, F, F, F) {
    let t8957 = t338 * t8794;
    let t8958 = t118 * t8957;
    let t8960 = t665 * t1614;
    let t8963 = t8936 * t321;
    let t8966 = t797 * t8884;
    let t8971 = t5148 * t8621;
    let t8973 = t5259 * t8649;
    (t8957, t8958, t8960, t8963, t8966, t8971, t8973)
}
