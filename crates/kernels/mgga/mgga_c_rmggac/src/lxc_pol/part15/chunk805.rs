//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 805/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk805<F: Float>(t1598: F, t16503: F, t16504: F, t8435: F, t10072: F, t34761: F, t1502: F, t2281: F, t35039: F, t34975: F, t552: F, t9145: F, t14237: F, t8430: F, t38523: F, t8368: F, t8568: F) -> (F, F, F, F, F, F, F) {
    let t45187 = t16503 * t16504 * t1598 * t8435;
    let t45189 = t34761 * t10072;
    let t45193 = t16503 * t35039 * t2281 * t1502;
    let t45197 = t34975 * t16504 * t552 * t9145;
    let t45201 = t16503 * t14237 * t2281 * t8430;
    let t45205 = t16503 * t35039 * t38523 * t8435;
    let t45207 = t8368 * t8568;
    (t45187, t45189, t45193, t45197, t45201, t45205, t45207)
}
