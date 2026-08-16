//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1092/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1092(t1241: f64, t22393: f64, t22113: f64, t491: f64, t1238: f64, t1761: f64, t19232: f64, t19234: f64, t19249: f64, t22004: f64, t22008: f64, t22328: f64, t22334: f64, t22337: f64, t4945: f64, t498: f64, t5055: f64, t6244: f64, t6268: f64) -> (f64, f64, f64) {
    let t22394 = t1241 * t22393;
    let t22398 = t22113 * t491;
    let t22408 = 6.0_f64 * t1238 * t22004 - 6.0_f64 * t1238 * t22008 - t1238 * t22394 - 3.0_f64 * t1761 * t19232 - 6.0_f64 * t1761 * t19234 - 3.0_f64 * t1761 * t19249 + t22328 * t498 + 3.0_f64 * t22334 * t498 + 3.0_f64 * t22337 * t498 + t22398 * t498 + 6.0_f64 * t4945 * t6244 - 3.0_f64 * t4945 * t6268 + 6.0_f64 * t5055 * t6244 - 3.0_f64 * t5055 * t6268;
    (t22394, t22398, t22408)
}
