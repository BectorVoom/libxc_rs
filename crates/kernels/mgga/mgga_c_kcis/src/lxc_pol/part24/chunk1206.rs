//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1206/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1206<F: Float>(t26972: F, t8083: F, t96005: F, t96018: F, t26966: F, t28214: F, t28219: F, t7784: F, t7772: F, t97024: F, t96940: F, t1250: F, t15198: F, t251: F) -> (F, F, F, F, F, F, F, F) {
    let t97166 = t8083 * t26972;
    let t97173 = F::new(0.15476481481481481481e-2) * t96005;
    let t97193 = F::new(0.23214722222222222222e-2) * t96018;
    let t97212 = t26966 * t28214;
    let t97248 = F::new(0.23168402777777777778e-3) * t28219 * t7784;
    let t97250 = F::new(0.30918233506944444444e-4) * t7772 * t97024;
    let t97265 = F::new(0.30918233506944444444e-4) * t7772 * t96940;
    let t97267 = t15198 * t251 * t1250;
    (t97166, t97173, t97193, t97212, t97248, t97250, t97265, t97267)
}
