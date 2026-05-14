//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 792/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk792<F: Float>(t11: F, t5: F, t5193: F, t5195: F, t7637: F, t7641: F, t8879: F, t9029: F, t9038: F, t9044: F, t9047: F, t9051: F, t9055: F, t9059: F, t9075: F, t9077: F) -> (F,) {
    let t9083 = -t5193 + 20.0 / 9.0 * t5195 + 40.0 / 9.0 * t7637 - t7641 - 5.0 / 3.0 * t8879 + 5.0 * t5 * t11 * t9029 - 45.0 * param_eta * (t9038 + t9044 + t9047 + t9051 + t9055 + t9059 + t9075 + t9077);
    (t9083,)
}
