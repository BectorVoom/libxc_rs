//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1335/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1335<F: Float>(t5748: F, t5929: F, t3738: F, t7332: F, t1395: F, t22422: F, t22364: F, t27544: F, t22649: F, t97706: F, t576: F, t5905: F, t97800: F) -> (F, F, F, F, F, F) {
    let t102875 = t5748 * t5929;
    let t102877 = t3738 * t7332;
    let t102879 = t1395 * t22422;
    let t102881 = t27544 * t22364;
    let t102883 = t97706 * t22649;
    let t102886 = t576 * t97800 * t5905;
    (t102875, t102877, t102879, t102881, t102883, t102886)
}
