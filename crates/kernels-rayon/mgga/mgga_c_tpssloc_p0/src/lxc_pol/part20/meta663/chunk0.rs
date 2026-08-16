//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2484/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2484(t14725: f64, t9288: f64, t136: f64, t3297: f64, t14748: f64, t2250: f64, t1113: f64, t14735: f64, t2244: f64, t4728: f64, t9258: f64, t43768: f64, t43770: f64, t43777: f64, t50846: f64, t50848: f64, t50851: f64, t50854: f64, t50859: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t50861 = t14725 * t9288;
    let t50863 = t136 * t3297 * t50861;
    let t50865 = t14748 * t2250;
    let t50867 = t136 * t1113 * t50865;
    let t50869 = t14735 * t2244;
    let t50871 = t136 * t1113 * t50869;
    let t50873 = t4728 * t9258;
    let t50875 = t136 * t1113 * t50873;
    let t50877 = -0.24528888888888888889e0_f64 * t50846 - 0.16557e0_f64 * t50848 + 0.82785e-1_f64 * t50851 + t50854 + 0.55190000000000000001e-1_f64 * t43768 - 0.33114e0_f64 * t43770 + t43777 - 0.27595e-1_f64 * t50859 - 0.99342e0_f64 * t50863 + 0.49671e0_f64 * t50867 + 0.149013e1_f64 * t50871 + 0.16557e0_f64 * t50875;
    (t50861, t50863, t50865, t50867, t50869, t50871, t50873, t50875, t50877)
}
