//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 932/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk932<F: Float>(t34975: F, t34976: F, t571: F, t8455: F, t1368: F, t16503: F, t3369: F, t9163: F, t2186: F, t9731: F, t2320: F, t38370: F) -> (F, F, F, F) {
    let t45499 = t34975 * t34976 * t571 * t8455;
    let t45503 = t16503 * t3369 * t1368 * t9163;
    let t45505 = t2186 * t9731;
    let t45507 = t38370 * t2320;
    (t45499, t45503, t45505, t45507)
}
