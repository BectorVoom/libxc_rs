//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1509/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1509(t13716: f64, t942: f64, t951: f64, t959: f64, t2940: f64, t4489: f64, t10523: f64, t1580: f64, t2933: f64, t1543: f64, t2791: f64, t2794: f64) -> (f64, f64, f64, f64) {
    let t13718 = t942 * t13716 * t951;
    let t13720 = 0.5848223622634646207e0_f64 * t959 * t13718;
    let t13722 = 0.23392894490538584828e1_f64 * t2940 * t4489;
    let t13723 = t10523 * t1580;
    let t13724 = t13723 * t2933;
    let t13726 = 0.10389515463408878255e3_f64 * t959 * t13724;
    let t13727 = t1543 * t2791;
    let t13729 = 2.0_f64 * t13727 * t2794;
    (t13720, t13722, t13726, t13729)
}
