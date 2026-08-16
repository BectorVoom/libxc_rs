//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 437/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk437(t1129: f64, t3422: f64, t1101: f64, t1130: f64, t282: f64, t3071: f64, t3075: f64, t3078: f64, t3130: f64, t3134: f64, t3142: f64, t3177: f64, t3366: f64, t3368: f64, t3373: f64, t3377: f64) -> (f64, f64) {
    let t3423 = t3422 * t1129;
    let t3435 = t3366 * t282 - 0.386e0_f64 * t3368 * t1130 + 0.74498e-1_f64 * t3373 * t3377 - 0.193e0_f64 * t1101 * t3423 + 0.193e0_f64 * t1101 * t3377 + 0.21667074074074074073e-1_f64 * t3071 - 0.18571777777777777777e-1_f64 * t3075 + 0.18571777777777777777e-1_f64 * t3078 + 0.69644166666666666665e-2_f64 * t3130 - 0.13928833333333333333e-1_f64 * t3134 + 0.13928833333333333333e-1_f64 * t3142 - 0.69644166666666666665e-2_f64 * t3177;
    (t3423, t3435)
}
