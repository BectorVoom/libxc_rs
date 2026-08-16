//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1208/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1208<F: Float>(t7884: F, t8172: F, t1181: F, t5885: F, t5888: F, t1867: F, t2994: F, t547: F, t8157: F, t8160: F, t19: F, t550: F, t8147: F) -> (F, F, F, F, F, F, F) {
    let t23083 = t7884 * t8172;
    let t23085 = t1181 * t5885;
    let t23087 = t1181 * t5888;
    let t23098 = t1867 * t2994;
    let t23100 = t547 * t8157;
    let t23102 = t547 * t8160;
    let t23105 = t19 * t550 * t8147;
    (t23083, t23085, t23087, t23098, t23100, t23102, t23105)
}
