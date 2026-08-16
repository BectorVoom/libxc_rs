//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 902/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk902(t2697: f64, t2703: f64, t842: f64, t9612: f64, t2617: f64, t2696: f64, t849: f64, t232: f64, t2553: f64, t2614: f64, t838: f64, t2693: f64, t809: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9988 = t2697 * t2703;
    let t9990 = t9612 * t842;
    let t9993 = t2617 * t2696;
    let t9994 = t9993 * t849;
    let t10007 = t232 * t2553;
    let t10012 = t2614 * t838;
    let t10014 = t809 * t2693;
    (t9988, t9990, t9993, t9994, t10007, t10012, t10014)
}
