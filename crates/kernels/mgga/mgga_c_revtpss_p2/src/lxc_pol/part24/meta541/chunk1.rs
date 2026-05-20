//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1590/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1590<F: Float>(t25072: F, t571: F, t5891: F, t5915: F, t5911: F, t5895: F, t5823: F, t5907: F, t22: F, t39454: F, t100: F, t105: F, t108: F, t109: F, t1507: F, t1510: F, t21835: F, t21860: F, t22604: F, t22608: F, t22618: F, t22621: F, t22624: F, t22625: F, t22699: F, t2349: F, t2357: F, t4269: F, t4279: F, t46196: F, t46212: F, t5902: F, t5908: F, t5912: F, t97: F, tau1: F) -> (F, F, F, F, F) {
    let t86909 = t571 * t25072;
    let t86981 = t5891 * t5891;
    let t86988 = t5915 * t5915;
    let t86994 = t5911 * t5911;
    let t87001 = t5895 * t5895;
    let t87008 = t5823 * t5823;
    let t87021 = t5907 * t5907;
    let t87028 = t22 + t39454;
    let t87029 = F::new(12.0) * t87028;
    let t87046 = F::new(10.0) / F::new(3.0) * t105 * t2357 * t86994 + F::new(40.0) / F::new(9.0) * t105 * t4279 * t22624 + F::new(40.0) / F::new(81.0) * t97 * t46196 * t87001 - F::new(20.0) / F::new(9.0) * t97 * t21835 * t5823 + F::new(10.0) / F::new(3.0) * t97 * t2349 * t87008 + F::new(40.0) / F::new(9.0) * t97 * t4269 * t22604 + F::new(800.0) / F::new(27.0) * t5902 * t5908 + F::new(200.0) / F::new(81.0) * t1507 * t22618 - F::new(200.0) / F::new(9.0) * t1507 * t22621 + F::new(40.0) / F::new(81.0) * t105 * t46212 * t87021 - F::new(20.0) / F::new(9.0) * t105 * t21860 * t5911 + F::new(5.0) / F::new(3.0) * t97 * t100 * t87029 + F::new(6160.0) / F::new(81.0) * tau1 * t22699 * t109 - F::new(8800.0) / F::new(81.0) * t22608 * t1510 + F::new(400.0) / F::new(9.0) * t5902 * t5912 - F::new(100.0) / F::new(9.0) * t1507 * t22625 - F::new(5.0) / F::new(3.0) * t105 * t108 * t87029;
    (t86909, t86981, t86988, t87028, t87046)
}
