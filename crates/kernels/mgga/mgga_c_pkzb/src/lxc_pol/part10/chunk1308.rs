//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1308/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1308<F: Float>(t1957: F, t730: F, t9351: F, t7269: F, t7483: F, t7279: F, t7411: F, t5771: F, t9216: F, t5734: F, t9219: F, t17329: F, t9222: F, t9225: F, t9229: F, t1855: F, t683: F, t9390: F) -> (F, F, F, F, F, F, F, F, F) {
    let t25855 = 0.35089341735807877242e1 * t730 * t9351 * t1957;
    let t25857 = 8.0 * t7483 * t7269;
    let t25859 = 0.64327917994770140268e2 * t7411 * t7279;
    let t25861 = 12.0 * t5771 * t9216;
    let t25863 = 8.0 * t5734 * t9219;
    let t25865 = 0.1929837539843104208e3 * t17329 * t9222;
    let t25867 = 4.0 * t5734 * t9225;
    let t25869 = 0.32163958997385070134e2 * t5771 * t9229;
    let t25872 = 4.0 * t1855 * t9390 * t683;
    (t25855, t25857, t25859, t25861, t25863, t25865, t25867, t25869, t25872)
}
