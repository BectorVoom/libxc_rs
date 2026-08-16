//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1187/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1187(t12747: f64, t1761: f64, t4396: f64, t5924: f64, t6265: f64, t1016: f64, t1838: f64, t1165: f64, t1173: f64, t1180: f64, t1181: f64, t16690: f64, t16692: f64, t16694: f64, t16701: f64, t16703: f64, t16705: f64, t4289: f64, t4314: f64, t4680: f64, t6269: f64, t6399: f64) -> f64 {
    let t21583 = t12747 * t1761;
    let t21592 = t4396 * t5924;
    let t21594 = t4396 * t6265;
    let t21596 = t1016 * t1838;
    let t21601 = -0.17149607247227894789e-2_f64 * t16690 - 0.34299214494455789578e-2_f64 * t16692 - 0.34299214494455789578e-2_f64 * t16694 - 0.13605355082800796533e0_f64 * t16701 - 0.90702367218671976884e-1_f64 * t16703 + 0.24009450146119052704e-1_f64 * t16705 + 0.22675591804667994221e-1_f64 * t21583 - 0.17149607247227894789e-2_f64 * t1180 * t4680 * t6399 + 0.68598428988911579156e-2_f64 * t1173 * t1181 * t4289 * t6269 - 0.17149607247227894789e-2_f64 * t21592 + 0.17149607247227894789e-2_f64 * t21594 - 0.12862205435420921092e-2_f64 * t1180 * t1165 * t21596 * t4314;
    t21601
}
