//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1914/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1914(t1599: f64, t6699: f64, t1922: f64, t4542: f64, t1625: f64, t6703: f64, t6706: f64, t7561: f64, t986: f64, t23365: f64, t7565: f64, t23336: f64, t7553: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t25400 = t1599 * t6699;
    let t25403 = t4542 * t1922;
    let t25406 = t6703 * t1625;
    let t25407 = t25406 * t6706;
    let t25410 = t986 * t7561;
    let t25413 = t23365 * t7565;
    let t25416 = t23336 * t7553;
    (t25400, t25403, t25406, t25407, t25410, t25413, t25416)
}
