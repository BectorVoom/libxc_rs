//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 861/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk861<F: Float>(t4873: F, t5039: F, t6039: F, t6047: F, t7156: F, t8653: F, t8654: F, t8655: F, t8656: F, t8657: F, t8658: F, t11: F, t5: F, t5193: F, t5195: F, t7637: F, t7641: F, t8879: F, t9029: F, t9038: F, t9044: F, t9047: F, t9051: F, t9055: F, t9059: F, t9075: F) -> F {
    let t9077 = t8653 + t8654 + t8655 - t4873 + F::new(0.285764e-1) * t6039 + t6047 + t7156 + t8656 + t8657 - t8658 - t5039;
    let t9083 = -t5193 + F::new(20.0) / F::new(9.0) * t5195 + F::new(40.0) / F::new(9.0) * t7637 - t7641 - F::new(5.0) / F::new(3.0) * t8879 + F::new(5.0) * t5 * t11 * t9029 - F::new(45.0) * param_eta * (t9038 + t9044 + t9047 + t9051 + t9055 + t9059 + t9075 + t9077);
    t9083
}
