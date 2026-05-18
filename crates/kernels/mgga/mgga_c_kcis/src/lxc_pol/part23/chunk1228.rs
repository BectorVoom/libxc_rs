//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1228/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1228<F: Float>(t1542: F, t1928: F, t1394: F, t7924: F, t16744: F, t491: F, t990: F, t17279: F, t27387: F, t1494: F, t2242: F, t15870: F, t5661: F) -> (F, F, F, F, F) {
    let t98020 = t1542 * t1928;
    let t98022 = t1394 * t98020 * t7924;
    let t98025 = t16744 * t491 * t990;
    let t98030 = t1394 * t27387 * t17279;
    let t98034 = t2242 * t1494;
    let t98036 = t5661 * t98034 * t15870;
    (t98022, t98025, t98030, t98034, t98036)
}
