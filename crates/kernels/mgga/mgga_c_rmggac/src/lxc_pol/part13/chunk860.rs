//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 860/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk860<F: Float>(t2410: F, t7228: F, t1969: F, t7457: F, t1212: F, t1970: F, t209: F, t236: F, t3352: F, t551: F, t1971: F, t5578: F) -> (F, F, F, F) {
    let t39207 = t2410 * t7228;
    let t39208 = t39207 * t1969;
    let t39209 = t39208 * t7457;
    let t39215 = t1970 * t3352 * t236 * t551 * t1212 * t209;
    let t39219 = t1970 * t1971 * t236 * t5578;
    (t39207, t39209, t39215, t39219)
}
