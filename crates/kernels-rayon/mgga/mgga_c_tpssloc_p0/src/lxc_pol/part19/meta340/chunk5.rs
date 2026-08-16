//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1214/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1214(t9638: f64, t9649: f64, t120: f64, t9957: f64, t13262: f64, t2623: f64, t2643: f64, t2645: f64, t2649: f64, t40848: f64, t40951: f64, t41039: f64, t41048: f64, t41050: f64, t41053: f64, t41055: f64, t41063: f64, t4178: f64, t4180: f64, t820: f64, t829: f64, t843: f64, t847: f64, t9623: f64, t9626: f64, t9627: f64, t9642: f64, t9997: f64) -> (f64, f64) {
    let t41066 = t9638 * t9649;
    let t41072 = t120 * t9957;
    let t41077 = -t4178 * t2645 * t41039 * t9627 / 32.0_f64 - 3.0_f64 / 256.0_f64 * t13262 * t4180 * t9626 * t40951 - 7.0_f64 / 96.0_f64 * t41048 - 7.0_f64 / 96.0_f64 * t41050 - 119.0_f64 / 288.0_f64 * t41053 + 7.0_f64 / 96.0_f64 * t41055 - t2623 * t9997 / 192.0_f64 - t843 * t847 * t820 * t40848 / 768.0_f64 + t41063 * t2649 / 64.0_f64 + 35.0_f64 / 96.0_f64 * t41066 - 5.0_f64 / 64.0_f64 * t9642 * t9649 - t9642 * t9623 / 256.0_f64 - t2643 * t4180 * t41072 * t829 / 768.0_f64;
    (t41072, t41077)
}
