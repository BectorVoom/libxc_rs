//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 875/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk875<F: Float>(t1037: F, t5384: F, t1769: F, t2663: F, t2575: F, t51: F, t2660: F, t568: F, t1692: F, t2661: F, t1727: F, t2642: F, t4886: F, t4867: F, t4870: F, t4876: F, t4879: F, t6762: F, t6800: F, t6803: F, t6805: F, t6810: F, t6812: F, t6813: F, t6820: F, t6822: F, t6824: F, t6826: F) -> (F, F, F, F, F, F, F, F) {
    let t6995 = t5384 * t1037;
    let t6998 = 0.40015750243531754508e-1 * t1769 * t2663;
    let t6999 = t51 * t2575;
    let t7001 = t2660 * t6999 * t568;
    let t7005 = t2660 * t2661 * t1692;
    let t7009 = 0.20007875121765877254e-2 * t1727 * t2642;
    let t7010 = 2.0 * t4886;
    let t7011 = t4867 + t4870 + t6762 - t4876 - t4879 + t6800 - t6803 + t6805 + t6810 + t6812 + t6813 + t6820 + t6822 - t6824 + t6826 + t7010;
    (t6995, t6998, t6999, t7001, t7005, t7009, t7010, t7011)
}
