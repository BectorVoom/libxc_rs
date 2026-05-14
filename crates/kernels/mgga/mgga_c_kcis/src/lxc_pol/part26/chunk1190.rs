//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1190/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1190<F: Float>(t22335: F, t27544: F, t20906: F, t97821: F, t22384: F, t7948: F, t5752: F, t5935: F, t5748: F, t5929: F, t3738: F, t7332: F, t1395: F, t22422: F, t22364: F, t22649: F, t97706: F) -> (F, F, F, F, F, F, F, F, F) {
    let t102867 = t27544 * t22335;
    let t102869 = t97821 * t20906;
    let t102871 = t7948 * t22384;
    let t102873 = t5752 * t5935;
    let t102875 = t5748 * t5929;
    let t102877 = t3738 * t7332;
    let t102879 = t1395 * t22422;
    let t102881 = t27544 * t22364;
    let t102883 = t97706 * t22649;
    (t102867, t102869, t102871, t102873, t102875, t102877, t102879, t102881, t102883)
}
