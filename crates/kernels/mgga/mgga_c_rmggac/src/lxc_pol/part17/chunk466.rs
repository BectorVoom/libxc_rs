//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 466/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk466<F: Float>(t1193: F, t5582: F, t1503: F, t31: F, t4518: F, t1466: F, t574: F, t934: F, t1710: F, t3869: F, t312: F, t50: F, t537: F) -> (F, F, F, F, F) {
    let t5694 = t1193 * t5582;
    let t5696 = F::cast_from(0.12805126321218922714e0_f64) * t5694 * t1503;
    let t5697 = t4518 * t31;
    let t5698 = t5697 * t1466;
    let t5757 = t934 * t574;
    let t5799 = t3869 * t1710;
    let t5800 = t5799 * t312;
    let t5803 = t537 * t50;
    (t5696, t5698, t5757, t5800, t5803)
}
