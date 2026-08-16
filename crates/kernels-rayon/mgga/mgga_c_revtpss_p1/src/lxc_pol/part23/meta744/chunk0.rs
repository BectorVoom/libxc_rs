//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2526/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2526(t51564: f64, t10115: f64, t1576: f64, t14593: f64, t2470: f64, t874: f64, t10538: f64, t14605: f64, t49180: f64, t10535: f64, t136: f64, t2457: f64, t4424: f64) -> (f64, f64, f64, f64, f64) {
    let t51565 = 0.34697458558045176417e-2_f64 * t51564;
    let t51578 = t10115 * t1576;
    let t51587 = t874 * t14593 * t2470;
    let t51588 = 0.39029762157531132076e-1_f64 * t51587;
    let t51603 = t49180 * t14605 * t10538;
    let t51604 = 0.34697458558045176417e-2_f64 * t51603;
    let t51614 = t10535 * t4424 * t136 * t2457;
    (t51565, t51578, t51588, t51604, t51614)
}
