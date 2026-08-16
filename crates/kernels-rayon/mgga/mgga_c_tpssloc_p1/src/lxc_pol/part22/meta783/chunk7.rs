//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2686/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2686(t1352: f64, t16233: f64, t16305: f64, t16394: f64, t19886: f64, t19894: f64, t19981: f64, t3803: f64, t40449: f64, t54013: f64, t54014: f64, t54786: f64, t54793: f64, t54812: f64, t56812: f64, t57091: f64, t57437: f64, t57450: f64, t57457: f64, t6394: f64, t74415: f64) -> f64 {
    let t74833 = -t3803 * t54013 * t74415 * t1352 / 1024.0_f64 + t3803 * t16305 * t57091 * t6394 / 256.0_f64 - 3.0_f64 / 512.0_f64 * t16233 * t54013 * t54014 * t56812 - 5.0_f64 / 256.0_f64 * t16394 * t19981 - 7.0_f64 / 384.0_f64 * t57437 + 7.0_f64 / 96.0_f64 * t57450 + 7.0_f64 / 768.0_f64 * t57457 + t54786 - 595.0_f64 / 3456.0_f64 * t54793 + t40449 + t54812 + t3803 * t16305 * t56812 * t6394 / 256.0_f64 - 5.0_f64 / 128.0_f64 * t16394 * t19894 + t16394 * t19886 / 128.0_f64;
    t74833
}
