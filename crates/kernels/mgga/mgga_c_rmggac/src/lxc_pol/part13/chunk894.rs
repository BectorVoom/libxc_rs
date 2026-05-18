//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 894/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk894<F: Float>(t333: F, t3351: F, t511: F, t7248: F, t9216: F, t352: F, t515: F, t1970: F, t1971: F, t236: F, t5601: F, t38350: F, t7473: F) -> (F, F, F, F) {
    let t39813 = t3351 * t7248 * t511 * t9216 * t333;
    let t39818 = t3351 * t7248 * t515 * t9216 * t352;
    let t39830 = t1970 * t1971 * t236 * t5601;
    let t39832 = t38350 * t7473;
    (t39813, t39818, t39830, t39832)
}
