//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 881/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk881(t812: f64, t9670: f64, t831: f64, t2617: f64, t2638: f64, t2639: f64, t2681: f64, t116: f64, t126: f64, t136: f64, t16: f64, t2386: f64, t625: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9671 = t812 * t9670;
    let t9672 = t9671 * t831;
    let t9674 = t2617 * t2638;
    let t9675 = t9674 * t831;
    let t9679 = t2639 * t2681;
    let t9688 = 1.0_f64 / t126 / t136 * t116 / 4.0_f64;
    let t9689 = t9688 * t16;
    let t9691 = t2386 * t625;
    (t9671, t9672, t9674, t9675, t9679, t9689, t9691)
}
