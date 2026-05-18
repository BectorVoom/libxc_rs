//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 913/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk913<F: Float>(t16504: F, t34975: F, t552: F, t9145: F, t14237: F, t16503: F, t2281: F, t8430: F, t35039: F, t38523: F, t8435: F, t8368: F, t8568: F) -> (F, F, F, F) {
    let t45197 = t34975 * t16504 * t552 * t9145;
    let t45201 = t16503 * t14237 * t2281 * t8430;
    let t45205 = t16503 * t35039 * t38523 * t8435;
    let t45207 = t8368 * t8568;
    (t45197, t45201, t45205, t45207)
}
