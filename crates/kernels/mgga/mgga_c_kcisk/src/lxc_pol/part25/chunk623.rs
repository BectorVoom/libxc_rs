//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 623/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk623<F: Float>(t227: F, t5821: F, t7: F, t171: F, t156: F, t3122: F, t119: F, t179: F, t139: F, t41: F, t339: F, t63: F, t67: F, t201: F, t5584: F, t565: F, t1757: F, t2063: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t228 = t227 <= zeta_threshold;
    let t5822 = t7 * t5821;
    let t5823 = t171 * t5822;
    let t5827 = t156 * t3122;
    let t5911 = t179 * t119;
    let t5913 = t139 * t5911 * t41;
    let t6141 = t339 * t63 * t67;
    let t6278 = t139 * t201 * t41;
    let t6650 = piecewise3(t228, 0.0, -t5584);
    let t6651 = t565 * t6650;
    let t6661 = t2063 * t1757;
    (t5822, t5823, t5827, t5911, t5913, t6141, t6278, t6650, t6651, t6661)
}
