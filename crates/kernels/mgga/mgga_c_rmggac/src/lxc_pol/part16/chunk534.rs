//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 534/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk534<F: Float>(t7651: F, t22: F, t3819: F, t2118: F, t7645: F, t344: F, t830: F, t1173: F, t2189: F, t674: F, t2064: F, t321: F, t1550: F, t201: F) -> (F, F, F, F, F, F, F, F, F) {
    let t7652 = 0.4838420607177634088e-3 * t7651;
    let t7653 = t3819 * t22;
    let t7656 = t2118 * t7645;
    let t7662 = t344 * t830;
    let t7690 = t2189 * t1173;
    let t7691 = t7690 * t674;
    let t7707 = t2064 * t321;
    let t7708 = t1550 * t7707;
    let t7715 = t201 * t1173;
    (t7652, t7653, t7656, t7662, t7690, t7691, t7707, t7708, t7715)
}
