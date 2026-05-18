//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1201/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1201<F: Float>(t26966: F, t28093: F, t7788: F, t96812: F, t95815: F, t27042: F, t46978: F, t8094: F, t95826: F, t1259: F, t417: F, t26954: F, t28189: F) -> (F, F, F, F, F, F, F, F) {
    let t96868 = F::new(0.61782407407407407408e-3) * t26966 * t28093;
    let t96875 = t7788 * t96812;
    let t96885 = F::new(0.15476481481481481481e-2) * t95815;
    let t96899 = t27042 * t28093;
    let t96902 = t7788 * t46978 * t8094;
    let t96904 = F::new(0.15476481481481481481e-2) * t95826;
    let t96908 = t417 * t1259;
    let t96917 = t28189 * t26954;
    (t96868, t96875, t96885, t96899, t96902, t96904, t96908, t96917)
}
