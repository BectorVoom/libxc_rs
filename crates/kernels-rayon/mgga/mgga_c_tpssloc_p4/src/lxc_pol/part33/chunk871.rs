//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 871/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk871(t1834: f64, t3787: f64, t111: f64, t1851: f64, t5520: f64, t751: f64, t5392: f64, t2658: f64, t5660: f64, t870: f64, t172: f64, t5522: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16428 = t3787 * t1834;
    let t16524 = t1851 * t111;
    let t16578 = t5520 * t751;
    let t16586 = t751 * t5392;
    let t16587 = t2658 * t16586;
    let t16606 = t5660 * t870;
    let t16616 = t5522 * t172;
    (t16428, t16524, t16578, t16587, t16606, t16616)
}
