//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1272/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1272(t15569: f64, t18371: f64, t19051: f64, t4993: f64, t11784: f64, t1227: f64, t21762: f64, t248: f64, t1174: f64, t135: f64, t22128: f64, t22132: f64) -> (f64, f64, f64, f64, f64) {
    let t72542 = t15569 * t18371;
    let t72556 = t19051 * t4993;
    let t72560 = t1227 * t248 * t11784 * t21762;
    let t72597 = t1174 * t135 * t22128;
    let t72600 = t1174 * t135 * t22132;
    (t72542, t72556, t72560, t72597, t72600)
}
