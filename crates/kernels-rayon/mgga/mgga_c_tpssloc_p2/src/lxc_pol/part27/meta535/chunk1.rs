//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1954/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1954(t33: f64, t7440: f64, t2240: f64, t1433: f64, t645: f64, t72: f64, t1865: f64, t22523: f64, t22554: f64, t26055: f64, t26063: f64, t26067: f64, t26070: f64, t26073: f64, t26076: f64, t6490: f64, t6492: f64, t6495: f64, t6506: f64, t6510: f64, t7432: f64, t7435: f64, t7442: f64, t7446: f64) -> (f64, f64, f64, f64) {
    let t26083 = t33 * t7440;
    let t26084 = t2240 * t26083;
    let t26090 = t72 * t1433 * t645;
    let t26095 = t26055 * t1865 / 3.0_f64 + 5.0_f64 / 6.0_f64 * t22554 * t7432 + 5.0_f64 / 6.0_f64 * t22523 * t7432 + 5.0_f64 / 6.0_f64 * t6490 * t26063 + 5.0_f64 / 6.0_f64 * t6490 * t26067 + t26070 * t1865 / 3.0_f64 + t26073 * t1865 / 3.0_f64 + t26076 * t1865 / 3.0_f64 + t7435 * t6506 / 3.0_f64 + t7435 * t6510 / 3.0_f64 + 5.0_f64 / 6.0_f64 * t26084 * t6492 + t6495 * t7442 / 3.0_f64 + 5.0_f64 / 6.0_f64 * t6490 * t26090 + t6495 * t7446 / 3.0_f64;
    (t26083, t26084, t26090, t26095)
}
