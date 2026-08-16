//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 1233/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk1233(t16205: f64, t550: f64, t1343: f64, t820: f64, t12365: f64, t1827: f64, t12300: f64, t1799: f64, t3734: f64, t12351: f64, t12418: f64, t1351: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t16206 = t16205 * t550;
    let t16208 = t1343 * t820 * t16206;
    let t16211 = t12365 * t1827;
    let t16214 = 7.0_f64 / 2304.0_f64 * t12300 * t1827;
    let t16215 = t1799 * t3734;
    let t16217 = t12351 * t820 * t16215;
    let t16224 = t12418 * t820;
    let t16225 = t1799 * t1351;
    (t16206, t16208, t16211, t16214, t16217, t16224, t16225)
}
