//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1432/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1432(t5151: f64, t67: f64, t758: f64, t12365: f64, t1827: f64, t12300: f64, t12418: f64, t820: f64, t1351: f64, t1799: f64, t12289: f64, t242: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16169 = t5151 * t67;
    let t16171 = 0.36622894612013090108e-3_f64 * t16169 * t758;
    let t16211 = t12365 * t1827;
    let t16214 = 7.0_f64 / 2304.0_f64 * t12300 * t1827;
    let t16224 = t12418 * t820;
    let t16225 = t1799 * t1351;
    let t16232 = t12289 * t242;
    (t16171, t16211, t16214, t16224, t16225, t16232)
}
