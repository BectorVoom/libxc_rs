//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1275/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1275(t11539: f64, t1174: f64, t21745: f64, t1213: f64, t22244: f64, t248: f64, t3570: f64, t1227: f64, t21758: f64, t45268: f64, t11692: f64, t11697: f64, t22283: f64) -> (f64, f64, f64, f64) {
    let t72815 = t1174 * t11539 * t21745;
    let t72849 = t1213 * t248 * t3570 * t22244;
    let t72857 = t1227 * t248 * t45268 * t21758;
    let t72864 = t11692 * t11697 * t22283;
    (t72815, t72849, t72857, t72864)
}
