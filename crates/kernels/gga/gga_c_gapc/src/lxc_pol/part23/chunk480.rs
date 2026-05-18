//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 480/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk480<F: Float>(t314: F, t5: F, t4: F, t291: F, t297: F, t512: F, t641: F, t916: F, t1044: F, t6: F, t442: F, t329: F) -> (F, F, F, F) {
    let t2693 = t314 * t5;
    let t2694 = t2693 * t4;
    let t2695 = t512 * t291 * t297 * t2694;
    let t2698 = t916 * t641;
    let t2699 = t1044 * t291;
    let t2701 = t314 * t6;
    let t2702 = t2701 * t442;
    let t2703 = t2699 * t329 * t2702;
    (t2695, t2698, t2701, t2703)
}
