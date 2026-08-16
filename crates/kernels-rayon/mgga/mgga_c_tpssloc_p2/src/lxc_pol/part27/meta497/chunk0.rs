//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1886/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1886(t1509: f64, t236: f64, t23110: f64, t232: f64, t23109: f64, t1898: f64, t4162: f64, t249: f64, t1496: f64, t23069: f64, t4257: f64, t6621: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t25130 = t236 * t1509;
    let t25132 = t23110 * t25130 * t232;
    let t25133 = t23109 * t25132;
    let t25135 = t4162 * t1898;
    let t25136 = t25135 * t249;
    let t25140 = t23069 * t1496;
    let t25142 = t6621 * t4257;
    (t25130, t25132, t25133, t25135, t25136, t25140, t25142)
}
