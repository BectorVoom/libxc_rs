//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 970/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk970<F: Float>(t2672: F, t5679: F, t1645: F, t1891: F, t6115: F, t935: F, t10913: F, t2021: F, t1980: F, t7512: F, t4370: F, t6109: F, t787: F, t1: F, t21888: F, t5654: F, t7809: F) -> (F, F, F, F, F, F, F, F) {
    let t22166 = t5679 * t2672;
    let t22213 = t1645 * t1891;
    let t22238 = t6115 * t935;
    let t22242 = t2021 * t10913;
    let t22263 = t1980 * t7512;
    let t22274 = t787 * t6109 * t4370;
    let t22295 = t787 * t21888 * t1;
    let t22315 = t5654 * t7809;
    (t22166, t22213, t22238, t22242, t22263, t22274, t22295, t22315)
}
