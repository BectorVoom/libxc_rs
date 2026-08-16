//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 830/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk830<F: Float>(t3351: F, t3352: F, t40983: F, t515: F, t15262: F, t16043: F, t14107: F, t5058: F, t14368: F, t15353: F, t14155: F, t56963: F) -> (F, F, F, F, F) {
    let t74913 = t3351 * t3352 * t515 * t40983;
    let t74915 = t16043 * t15262;
    let t74917 = t5058 * t14107;
    let t74919 = t14368 * t15353;
    let t74921 = t56963 * t14155;
    (t74913, t74915, t74917, t74919, t74921)
}
