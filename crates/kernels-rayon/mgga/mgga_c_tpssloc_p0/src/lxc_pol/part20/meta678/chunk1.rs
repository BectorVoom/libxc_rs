//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2562/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2562(t11366: f64, t1164: f64, t14853: f64, t11129: f64, t1694: f64, t43689: f64, t43692: f64, t11400: f64, t4874: f64, t11365: f64, t300: f64, t4861: f64, t51811: f64) -> (f64, f64, f64, f64) {
    let t51839 = 0.6233709278045326953e3_f64 * t1164 * t14853 * t11366;
    let t51844 = 0.91082604192152556044e5_f64 * t1164 * t43689 * t1694 * t43692 * t11129;
    let t51847 = 0.11696447245269292414e1_f64 * t1164 * t4874 * t11400;
    let t51848 = t300 * t11365;
    let t51851 = 0.31168546390226634765e3_f64 * t51848 * t4861 * t51811;
    (t51839, t51844, t51847, t51851)
}
