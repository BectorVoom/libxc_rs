//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 877/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk877(t12289: f64, t242: f64, t1336: f64, t3789: f64, t5234: f64, t3798: f64, t3804: f64, t820: f64, t1824: f64, t3792: f64, t12345: f64, t1831: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16232 = t12289 * t242;
    let t16233 = t1336 * t16232;
    let t16285 = t5234 * t3789;
    let t16288 = t5234 * t3798;
    let t16305 = t3804 * t820;
    let t16311 = t1824 * t3792;
    let t16317 = t12345 * t1831;
    (t16233, t16285, t16288, t16305, t16311, t16317)
}
