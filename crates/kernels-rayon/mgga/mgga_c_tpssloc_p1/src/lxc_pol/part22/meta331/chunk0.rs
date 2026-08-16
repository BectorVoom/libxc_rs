//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1518/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1518(t5151: f64, t67: f64, t758: f64, t1345: f64, t68: f64, t1799: f64, t1995: f64, t1365: f64, t5187: f64, t12365: f64, t1827: f64, t12300: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t16169 = t5151 * t67;
    let t16171 = 0.36622894612013090108e-3_f64 * t16169 * t758;
    let t16186 = t1345 * t68;
    let t16191 = t1995 * t1799;
    let t16195 = t1365 * t5187;
    let t16211 = t12365 * t1827;
    let t16214 = 7.0_f64 / 2304.0_f64 * t12300 * t1827;
    (t16169, t16171, t16186, t16191, t16195, t16211, t16214)
}
