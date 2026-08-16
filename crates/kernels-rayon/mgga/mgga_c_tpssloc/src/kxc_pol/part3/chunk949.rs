//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 949/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk949(t11651: f64, t3515: f64, t3576: f64, t3604: f64, t3585: f64, t820: f64, t10401: f64, t3575: f64, t3610: f64, t3624: f64, t3521: f64, t3579: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11652 = t3515 * t11651;
    let t11665 = t3604 * t3576;
    let t11668 = t820 * t3585;
    let t11677 = t3575 * t10401;
    let t11678 = t3610 * t11677;
    let t11692 = t3624 * t11677;
    let t11697 = t820 * t3521;
    let t11698 = t11697 * t3579;
    (t11652, t11665, t11668, t11678, t11692, t11697, t11698)
}
