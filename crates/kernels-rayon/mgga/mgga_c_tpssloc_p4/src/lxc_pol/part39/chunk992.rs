//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 992/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk992(t11717: f64, t1210: f64, t11713: f64, t248: f64, t3509: f64, t3570: f64, t3506: f64, t135: f64, t3561: f64, t1174: f64, t3247: f64, t415: f64) -> (f64, f64, f64, f64) {
    let t11737 = t1210 * t11717;
    let t11738 = t11713 * t11737;
    let t11745 = t248 * t3570 * t3509;
    let t11746 = t3506 * t11745;
    let t11754 = t135 * t3561;
    let t11755 = t1174 * t11754;
    let t11778 = 1.0_f64 / t415 / t3247;
    (t11738, t11746, t11755, t11778)
}
