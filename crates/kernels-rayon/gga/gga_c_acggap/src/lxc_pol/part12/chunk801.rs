//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 801/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk801(t1181: f64, t8774: f64, t7337: f64, t5606: f64, t604: f64, t2068: f64, t1165: f64, t5720: f64, t7351: f64, t524: f64, t944: f64, t406: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8775 = t1181 * t8774;
    let t8776 = t7337 * t8775;
    let t8778 = t604 * t5606;
    let t8779 = t1181 * t8778;
    let t8780 = t2068 * t8779;
    let t8783 = t1165 * t604 * t5720;
    let t8784 = t7337 * t8783;
    let t8787 = t1165 * t7351 * t5606;
    let t8788 = t2068 * t8787;
    let t8790 = t524 * t944;
    let t8791 = t8790 * t406;
    (t8775, t8776, t8778, t8779, t8780, t8783, t8784, t8787, t8788, t8790, t8791)
}
