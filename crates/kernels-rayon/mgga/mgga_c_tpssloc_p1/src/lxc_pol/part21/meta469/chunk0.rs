//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2046/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2046(t1343: f64, t16206: f64, t820: f64, t12365: f64, t1827: f64, t12300: f64, t1799: f64, t3734: f64, t12351: f64, t12418: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16208 = t1343 * t820 * t16206;
    let t16211 = t12365 * t1827;
    let t16214 = 7.0_f64 / 2304.0_f64 * t12300 * t1827;
    let t16215 = t1799 * t3734;
    let t16217 = t12351 * t820 * t16215;
    let t16224 = t12418 * t820;
    (t16208, t16211, t16214, t16215, t16217, t16224)
}
