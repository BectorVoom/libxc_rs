//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1393/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1393(t11745: f64, t3506: f64, t135: f64, t3561: f64, t1174: f64, t3247: f64, t415: f64, t121: f64, t3584: f64, t248: f64, t3243: f64, t1227: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11746 = t3506 * t11745;
    let t11754 = t135 * t3561;
    let t11755 = t1174 * t11754;
    let t11778 = 1.0_f64 / t415 / t3247;
    let t11784 = t121 * t3584;
    let t11786 = t248 * t11784 * t3243;
    let t11787 = t1227 * t11786;
    (t11746, t11754, t11755, t11778, t11786, t11787)
}
