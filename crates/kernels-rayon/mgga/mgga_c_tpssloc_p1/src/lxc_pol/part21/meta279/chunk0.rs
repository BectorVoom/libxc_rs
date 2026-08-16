//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1560/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1560(t849: f64, t9993: f64, t232: f64, t2553: f64, t2614: f64, t838: f64, t2693: f64, t809: f64, t597: f64, t61: f64) -> (f64, f64, f64, f64, f64) {
    let t9994 = t9993 * t849;
    let t10007 = t232 * t2553;
    let t10012 = t2614 * t838;
    let t10014 = t809 * t2693;
    let t10021 = 1.0_f64 / t61 / t597;
    (t9994, t10007, t10012, t10014, t10021)
}
