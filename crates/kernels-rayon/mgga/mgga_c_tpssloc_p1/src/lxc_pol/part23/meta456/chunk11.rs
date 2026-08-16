//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1330/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1330(t16606: f64, t16625: f64, t193: f64, t2378: f64, t2522: f64, t39658: f64, t41258: f64, t41262: f64, t4314: f64, t5527: f64, t5544: f64, t68371: f64, t76026: f64, t76027: f64, t76030: f64, t76031: f64, t76034: f64, t76035: f64, t76037: f64, t76063: f64) -> f64 {
    let t76556 = 18.0_f64 * t16606 * t2522 * t5544 - 36.0_f64 * t16625 * t4314 * t5527 + 18.0_f64 * t193 * t2378 * t76063 + 36.0_f64 * t193 * t5544 * t68371 - t39658 - t41258 - t41262 - t76026 + t76027 + t76030 + t76031 + t76034 - t76035 + t76037;
    t76556
}
