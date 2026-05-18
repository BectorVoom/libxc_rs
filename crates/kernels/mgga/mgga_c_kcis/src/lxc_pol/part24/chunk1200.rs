//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1200/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1200<F: Float>(t95587: F, t1250: F, t251: F, t47652: F, t2888: F, t7773: F, t46978: F, t8086: F, t7772: F, t15553: F, t28145: F, t7788: F) -> (F, F, F, F, F, F) {
    let t96787 = F::new(0.15476481481481481481e-2) * t95587;
    let t96790 = t47652 * t251 * t1250;
    let t96793 = t2888 * t7773;
    let t96812 = t46978 * t8086;
    let t96813 = t7772 * t96812;
    let t96836 = t7788 * t15553 * t28145;
    (t96787, t96790, t96793, t96812, t96813, t96836)
}
