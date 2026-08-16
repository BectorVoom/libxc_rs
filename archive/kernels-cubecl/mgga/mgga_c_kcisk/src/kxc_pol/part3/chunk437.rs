//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 437/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk437<F: Float>(t1129: F, t3422: F, t1101: F, t1130: F, t282: F, t3071: F, t3075: F, t3078: F, t3130: F, t3134: F, t3142: F, t3177: F, t3366: F, t3368: F, t3373: F, t3377: F) -> (F, F) {
    let t3423 = t3422 * t1129;
    let t3435 = t3366 * t282 - F::cast_from(0.386e0_f64) * t3368 * t1130 + F::cast_from(0.74498e-1_f64) * t3373 * t3377 - F::cast_from(0.193e0_f64) * t1101 * t3423 + F::cast_from(0.193e0_f64) * t1101 * t3377 + F::cast_from(0.21667074074074074073e-1_f64) * t3071 - F::cast_from(0.18571777777777777777e-1_f64) * t3075 + F::cast_from(0.18571777777777777777e-1_f64) * t3078 + F::cast_from(0.69644166666666666665e-2_f64) * t3130 - F::cast_from(0.13928833333333333333e-1_f64) * t3134 + F::cast_from(0.13928833333333333333e-1_f64) * t3142 - F::cast_from(0.69644166666666666665e-2_f64) * t3177;
    (t3423, t3435)
}
