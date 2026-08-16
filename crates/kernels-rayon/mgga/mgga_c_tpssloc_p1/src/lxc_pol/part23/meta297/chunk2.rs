//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1023/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1023(t17817: f64, t4531: f64, t17804: f64, t4514: f64, t10295: f64, t13642: f64, t17286: f64, t17288: f64, t17290: f64, t21120: f64, t21132: f64, t21136: f64, t21140: f64, t21161: f64, t21168: f64) -> (f64, f64, f64) {
    let t21430 = t4531 * t17817;
    let t21433 = t17804 * t4514;
    let t21444 = t10295 + 5.0_f64 / 9.0_f64 * t13642 - t17286 / 9.0_f64 + 2.0_f64 / 3.0_f64 * t17288 - t17290 / 3.0_f64 + 2.0_f64 / 27.0_f64 * t21132 - t21120 / 3.0_f64 + t21168 / 6.0_f64 + t21140 - t21161 + t21136 / 6.0_f64;
    (t21430, t21433, t21444)
}
