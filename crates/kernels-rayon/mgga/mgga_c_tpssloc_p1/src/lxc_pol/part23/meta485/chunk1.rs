//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1485/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1485(t12571: f64, t1437: f64, t19299: f64, t20201: f64, t20204: f64, t20288: f64, t2240: f64, t39030: f64, t39032: f64, t39034: f64, t39036: f64, t39038: f64, t39040: f64, t39043: f64, t39063: f64, t3953: f64, t45844: f64, t5389: f64, t5445: f64, t55921: f64, t605: f64, t75284: f64, t79579: f64, t79585: f64, t79637: f64, t79707: f64, t86: f64, t9239: f64) -> f64 {
    let t79711 = (t39030 + t39032 + t39034 + t39036 + t39038 + t39040 + t39043) * t86 - 16.0_f64 * t75284 * t1437 + 120.0_f64 * t55921 * t5389 - 24.0_f64 * t19299 * t5445 - 480.0_f64 * t45844 * t20201 + 240.0_f64 * t12571 * t20204 - 16.0_f64 * t3953 * t20288 + 840.0_f64 * t39063 * t79579 - 720.0_f64 * t9239 * t5389 * t5445 + 60.0_f64 * t2240 * t79585 + 80.0_f64 * t2240 * t1437 * t20288 - 4.0_f64 * t605 * (t79637 + t79707);
    t79711
}
