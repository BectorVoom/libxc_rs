//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 783/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk783<F: Float>(t4131: F, t676: F, t123: F, t3915: F, t2453: F, t3914: F, t1444: F, t2438: F, t138: F, t4075: F, t556: F, t786: F, t4077: F, t2434: F, t1424: F, t4071: F, t4132: F, t9632: F, t9636: F, t9639: F, t9642: F, t9650: F, t9652: F, t9659: F, t9666: F, t9668: F) -> (F, F, F, F, F, F, F) {
    let t9670 = t676 * t4131;
    let t9671 = t123 * t9670;
    let t9672 = t3915 * t9671;
    let t9674 = t2453 * t3914;
    let t9675 = t2438 * t1444;
    let t9676 = t138 * t9675;
    let t9677 = t9674 * t9676;
    let t9679 = t556 * t4075;
    let t9680 = t786 * t9679;
    let t9681 = t676 * t4077;
    let t9682 = t123 * t9681;
    let t9683 = t9680 * t9682;
    let t9685 = t2434 * t1444;
    let t9686 = t123 * t9685;
    let t9687 = t3915 * t9686;
    let t9689 = 0.21951497276451705329e-1 * t9632 - 0.16463622957338778996e-1 * t9636 + t9639 - 0.19514881078765566038e-2 * t9642 + t9650 + 0.39512695097613069591e1 * t1424 * t9652 - 0.39512695097613069591e1 * t1424 * t9659 - 0.19756347548806534796e1 * t4071 * t4132 - t9666 + 0.16463622957338778996e-1 * t9668 - 0.29272321618148349057e-1 * t9672 - 0.34697458558045176417e-2 * t9677 + 0.58544643236296698113e-1 * t9683 + 0.39029762157531132076e-1 * t9687;
    (t9671, t9675, t9676, t9682, t9685, t9686, t9689)
}
