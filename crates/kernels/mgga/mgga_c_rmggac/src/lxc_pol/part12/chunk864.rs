//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 864/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk864<F: Float>(t2347: F, t876: F, t262: F, t34938: F, t7501: F, t8672: F, t321: F, t8704: F, t8640: F, t333: F, t7198: F, t352: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t39044 = t2347 * t876;
    let t39045 = t262 * t39044;
    let t39046 = t34938 * t39045;
    let t39048 = t7501 * t8672;
    let t39055 = t8704 * t321;
    let t39056 = t262 * t39055;
    let t39057 = t8640 * t39056;
    let t39059 = t8704 * t333;
    let t39060 = t262 * t39059;
    let t39061 = t7198 * t39060;
    let t39063 = t8704 * t352;
    (t39044, t39045, t39046, t39048, t39055, t39056, t39057, t39059, t39060, t39061, t39063)
}
