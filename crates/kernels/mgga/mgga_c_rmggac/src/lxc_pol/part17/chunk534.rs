//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 534/964 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk534<F: Float>(t22: F, t3819: F, t2118: F, t7645: F, t344: F, t830: F, t1173: F, t2189: F, t674: F, t4616: F, t664: F) -> (F, F, F, F, F, F) {
    let t7653 = t3819 * t22;
    let t7656 = t2118 * t7645;
    let t7662 = t344 * t830;
    let t7663 = 0.64905642291407286545e-3 * t7662;
    let t7690 = t2189 * t1173;
    let t7691 = t7690 * t674;
    let t7703 = t4616 * t664;
    (t7653, t7656, t7663, t7690, t7691, t7703)
}
