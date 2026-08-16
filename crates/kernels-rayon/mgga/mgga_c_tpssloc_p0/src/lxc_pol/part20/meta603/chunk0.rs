//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2183/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2183(t1174: f64, t11765: f64, t135: f64, t3551: f64, t698: f64, t3242: f64, t415: f64, t42341: f64, t44696: f64, t42344: f64, t483: f64, t1210: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t44803 = t1174 * t135 * t11765;
    let t44811 = t1174 * t698 * t3551;
    let t44827 = 1.0_f64 / t415 / t3242;
    let t44833 = t44696 * t42341;
    let t44834 = t483 * t42344;
    let t44836 = t44833 * t1210 * t44834;
    (t44803, t44811, t44827, t44833, t44834, t44836)
}
