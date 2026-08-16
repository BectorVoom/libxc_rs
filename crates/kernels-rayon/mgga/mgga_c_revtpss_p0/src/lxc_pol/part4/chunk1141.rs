//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1141/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1141(t2470: f64, t5721: f64, t3915: f64, t1445: f64, t5599: f64, t689: f64, t2435: f64, t5600: f64, t1426: f64, t1893: f64, t786: f64, t3917: f64) -> (f64, f64, f64, f64) {
    let t14090 = t5721 * t2470;
    let t14091 = t3915 * t14090;
    let t14094 = t5599 * t1445;
    let t14096 = 0.10975748638225852664e-1_f64 * t689 * t14094;
    let t14097 = t2435 * t5600;
    let t14099 = t1893 * t1426;
    let t14100 = t786 * t14099;
    let t14102 = 0.19514881078765566038e-1_f64 * t14100 * t3917;
    (t14091, t14096, t14097, t14102)
}
