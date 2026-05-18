//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 602/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk602<F: Float>(t4669: F, t7193: F, t5271: F, t7199: F, t5259: F, t7205: F, t3814: F, t7710: F, t5245: F, t645: F, t739: F, t7855: F) -> (F, F, F, F, F, F) {
    let t7863 = t4669 * t7193;
    let t7865 = t5271 * t7199;
    let t7867 = t5259 * t7205;
    let t7869 = t3814 * t7710;
    let t7877 = t5245 * t645;
    let t7897 = t739 * t7855;
    (t7863, t7865, t7867, t7869, t7877, t7897)
}
