//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1447/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1447(t78423: f64, t78441: f64, t78460: f64, t78489: f64, t78516: f64, t78545: f64, t78578: f64, t78634: f64, t1238: f64, t1751: f64, t1760: f64, t1761: f64, t19232: f64, t19234: f64, t22004: f64, t22008: f64, t22113: f64, t22393: f64, t22394: f64, t3598: f64, t491: f64, t4945: f64, t498: f64, t5055: f64, t6150: f64, t6238: f64, t6244: f64, t6268: f64, t73900: f64, t78379: f64) -> (f64, f64) {
    let t78637 = t78423 + t78441 + t78460 + t78489 + t78516 + t78545 + t78578 + t78634;
    let t78646 = 8.0_f64 * t1238 * t1760 * t22393 * t3598 + 6.0_f64 * t1238 * t3598 * t78379 + 4.0_f64 * t1751 * t22113 * t498 + t491 * t498 * t78637 + 6.0_f64 * t498 * t6150 * t6238 - 4.0_f64 * t1761 * t73900 - 6.0_f64 * t19232 * t6268 + 24.0_f64 * t19234 * t6244 - 12.0_f64 * t19234 * t6268 + 24.0_f64 * t22004 * t4945 + 24.0_f64 * t22004 * t5055 - 24.0_f64 * t22008 * t4945 - 4.0_f64 * t22394 * t5055;
    (t78637, t78646)
}
