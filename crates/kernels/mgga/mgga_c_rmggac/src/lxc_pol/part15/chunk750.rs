//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 750/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk750<F: Float>(t1323: F, t35206: F, t7761: F, t7556: F, t934: F, t270: F, t356: F, t290: F, t31: F, t2019: F, t640: F, t7764: F) -> (F, F, F, F, F, F, F, F) {
    let t35207 = t1323 * t35206;
    let t35208 = t35207 * t7761;
    let t35210 = t934 * t7556;
    let t35214 = t356 * t270;
    let t35215 = t290 * t35214;
    let t35219 = t356 * t31;
    let t35220 = t290 * t35219;
    let t35226 = t2019 * t7764 * t640 * t35214;
    (t35207, t35208, t35210, t35214, t35215, t35219, t35220, t35226)
}
