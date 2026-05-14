//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1246/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1246<F: Float>(t143: F, t30444: F, t3244: F, t3263: F, t10762: F, t10823: F, t173: F, t178: F, t180: F, t181: F, t2122: F, t2135: F, t2143: F, t3272: F, t4026: F, t4032: F, t4048: F, t6620: F, t746: F, t750: F, t8761: F, t8802: F, t8803: F, t8809: F, t8822: F, t8825: F) -> (F, F, F, F) {
    let t145 = 0.135e1 < t143;
    let t30500 = piecewise3(t145, 0.0, t30444);
    let t30518 = t3244 * t3244;
    let t30561 = t3263 * t3244;
    let t30575 = -4.0 * t2135 * t30518 * t180 + t2143 * t30518 * t180 / 2.0 - 2.0 * t178 * t30518 * t180 - 8.0 * t10823 * t8761 - 4.0 * t8825 * t4026 - 8.0 * t3272 * t10762 - 4.0 * t750 * t30500 - t173 * t30500 * t180 + t746 * t30500 * t180 / 2.0 - 8.0 * t30518 * t181 + 30.0 * t8802 * t30561 - 10.0 * t8809 * t30561 + t8822 * t30561 / 2.0 - 75.0 / 2.0 * t4048 * t8803 + 15.0 / 2.0 * t2122 * t4026 * t8803 + 15.0 / 2.0 * t4032 * t6620;
    (t30500, t30518, t30561, t30575)
}
