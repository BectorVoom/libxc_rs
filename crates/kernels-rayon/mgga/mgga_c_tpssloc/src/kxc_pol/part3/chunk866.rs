//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 866/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk866(t3946: f64, t576: f64, t16: f64, t2: f64, t591: f64, t9: f64, t21: f64, t587: f64, t14: f64, t598: f64, t2230: f64, t594: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9207 = t576 * t3946;
    let t9212 = t2 * t16;
    let t9214 = t9 * t591;
    let t9216 = t587 * t21;
    let t9218 = t14 * t598;
    let t9220 = t594 * t2230;
    (t9207, t9212, t9214, t9216, t9218, t9220)
}
