//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 734/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk734<F: Float>(t201: F, t4443: F, t1976: F, t674: F, t16156: F, t7251: F, t7738: F, t7376: F, t7244: F, t7259: F, t7541: F, t7715: F) -> (F, F, F, F, F, F, F) {
    let t34855 = t201 * t4443;
    let t34857 = t1976 * t34855 * t674;
    let t34869 = t16156 * t7251;
    let t34871 = t16156 * t7738;
    let t34873 = t16156 * t7376;
    let t34875 = t7244 * t7259;
    let t34878 = t7541 * t7715 * t674;
    (t34855, t34857, t34869, t34871, t34873, t34875, t34878)
}
