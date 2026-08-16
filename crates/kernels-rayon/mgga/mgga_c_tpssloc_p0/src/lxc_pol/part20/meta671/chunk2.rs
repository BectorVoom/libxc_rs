//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2523/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2523(t50853: f64, t43768: f64, t43770: f64, t44027: f64, t50846: f64, t50848: f64, t50851: f64, t50859: f64, t50863: f64, t50867: f64, t50871: f64, t50875: f64) -> f64 {
    let t51151 = 0.27385555555555555556e0_f64 * t50853;
    let t51159 = -0.24342716049382716049e0_f64 * t50846 - 0.16431333333333333333e0_f64 * t50848 + 0.82156666666666666667e-1_f64 * t50851 + t51151 + 0.54771111111111111111e-1_f64 * t43768 - 0.32862666666666666666e0_f64 * t43770 + t44027 - 0.27385555555555555556e-1_f64 * t50859 - 0.98587999999999999998e0_f64 * t50863 + 0.49293999999999999999e0_f64 * t50867 + 0.147882e1_f64 * t50871 + 0.16431333333333333333e0_f64 * t50875;
    t51159
}
