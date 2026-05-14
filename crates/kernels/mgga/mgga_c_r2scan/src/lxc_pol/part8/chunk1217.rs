//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1217/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1217<F: Float>(t1575: F, t2102: F, t25826: F, t571: F, t20994: F, t2553: F, t122: F, t512: F, t10855: F, t110: F, t20096: F, t7418: F, t20818: F, t24999: F, t5100: F, t7470: F) -> (F, F, F, F, F, F, F) {
    let t25827 = t571 * t1575 * t2102 * t25826;
    let t25835 = t20994 * t2553;
    let t25836 = 0.12805040077930161442e1 * t25835;
    let t25850 = t512 * t122;
    let t25851 = t10855 * t110;
    let t25852 = t25850 * t25851;
    let t25855 = t20096 * t7418;
    let t25871 = t20818 * t24999;
    let t25872 = 0.86743646395112941037e-3 * t25871;
    let t25951 = t5100 * t7470;
    (t25827, t25836, t25850, t25852, t25855, t25872, t25951)
}
