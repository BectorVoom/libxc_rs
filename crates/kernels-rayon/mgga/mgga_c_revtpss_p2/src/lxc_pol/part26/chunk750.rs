//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 750/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk750(t1424: f64, t4071: f64, t4132: f64, t9632: f64, t9636: f64, t9639: f64, t9642: f64, t9650: f64, t9652: f64, t9659: f64, t9666: f64, t9668: f64, t9672: f64, t9677: f64, t9683: f64, t9687: f64) -> f64 {
    let t9689 = 0.21951497276451705329e-1_f64 * t9632 - 0.16463622957338778996e-1_f64 * t9636 + t9639 - 0.19514881078765566038e-2_f64 * t9642 + t9650 + 0.39512695097613069591e1_f64 * t1424 * t9652 - 0.39512695097613069591e1_f64 * t1424 * t9659 - 0.19756347548806534796e1_f64 * t4071 * t4132 - t9666 + 0.16463622957338778996e-1_f64 * t9668 - 0.29272321618148349057e-1_f64 * t9672 - 0.34697458558045176417e-2_f64 * t9677 + 0.58544643236296698113e-1_f64 * t9683 + 0.39029762157531132076e-1_f64 * t9687;
    t9689
}
