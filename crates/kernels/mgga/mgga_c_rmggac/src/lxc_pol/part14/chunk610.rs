//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 610/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk610<F: Float>(t338: F, t7667: F, t118: F, t4669: F, t7193: F, t5271: F, t7199: F, t5259: F, t7205: F, t3814: F, t7710: F, t5245: F, t645: F) -> (F, F, F, F, F, F, F) {
    let t7858 = t338 * t7667;
    let t7859 = t118 * t7858;
    let t7863 = t4669 * t7193;
    let t7865 = t5271 * t7199;
    let t7867 = t5259 * t7205;
    let t7869 = t3814 * t7710;
    let t7877 = t5245 * t645;
    (t7858, t7859, t7863, t7865, t7867, t7869, t7877)
}
