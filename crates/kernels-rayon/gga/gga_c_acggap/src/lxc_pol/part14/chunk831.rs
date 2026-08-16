//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 831/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk831(t7575: f64, t9670: f64, t1165: f64, t1844: f64, t604: f64, t2068: f64, t1856: f64, t2001: f64, t7520: f64, t7540: f64, t7546: f64, t7550: f64, t7558: f64, t7602: f64, t7612: f64, t7632: f64, t7639: f64, t7641: f64, t7672: f64, t9292: f64, t9309: f64, t9661: f64, t9664: f64, t9667: f64) -> (f64, f64) {
    let t9671 = t7575 * t9670;
    let t9674 = t1165 * t604 * t1844;
    let t9675 = t2068 * t9674;
    let t9677 = t2001 * t1856;
    let t9679 = -t7520 + t7540 + t7546 + t7550 - t7558 - 0.4584375e-1_f64 * t9661 + 0.22921875e-1_f64 * t9664 + 0.1528125e-1_f64 * t9667 + 0.21437009059034868486e-2_f64 * t9671 - t7602 + t7612 + t7632 + t7639 - t7641 - t9292 + 0.15724046144802076034e-3_f64 * t9675 - t9309 + t7672 - 0.34299214494455789578e-2_f64 * t9677;
    (t9674, t9679)
}
