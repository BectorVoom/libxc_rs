//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1256/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1256<F: Float>(t20362: F, t46: F, t552: F, t8748: F, t16933: F, t20366: F, t20368: F, t20370: F, t20372: F, t20374: F, t16935: F, t1545: F, t3426: F, t1548: F, t20378: F, t16810: F, t16813: F, t16822: F, t16825: F, t16946: F, t16950: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t24660 = 32.0 * t20362;
    let t24662 = t8748 * t46 * t552;
    let t24663 = 0.36622894612013090108e-3 * t24662;
    let t24664 = 192.0 * t16933;
    let t24665 = 0.43374325201206959368e-1 * t20366;
    let t24666 = 0.65061487801810439052e-1 * t20368;
    let t24667 = 0.70178683471615754484e1 * t20370;
    let t24668 = 0.97661052298701573622e-3 * t20372;
    let t24669 = 0.10389515463408878255e3 * t20374;
    let t24670 = 0.11393789434848516922e-2 * t16935;
    let t24671 = t1545 * t3426;
    let t24672 = 12.0 * t24671;
    let t24673 = t1548 * t3426;
    let t24674 = 32.0 * t24673;
    let t24675 = 120.0 * t20378;
    let t24676 = -t24660 + t16810 - t16813 - t16822 - t24663 + t24664 - t24665 - t24666 + t24667 + t24668 - t24669 + t16825 - t24670 + t16946 + t16950 + t24672 - t24674 + t24675;
    (t24660, t24663, t24664, t24665, t24666, t24667, t24668, t24669, t24670, t24672, t24674, t24675, t24676)
}
