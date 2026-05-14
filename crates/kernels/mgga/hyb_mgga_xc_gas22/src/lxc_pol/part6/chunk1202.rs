//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1202/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1202<F: Float>(t3: F, t675: F, t3141: F, t10158: F, t10190: F, t10191: F, t10195: F, t10325: F, t2002: F, t20171: F, t2033: F, t214: F, t23783: F, t23788: F, t23791: F, t23930: F, t27852: F, t27857: F, t27871: F, t27880: F, t3138: F, t3140: F, t3150: F, t3938: F, t6457: F, t684: F, t687: F, t8502: F, t8511: F, t8513: F, t8514: F, t8519: F, t8526: F, t8561: F) -> (F, F) {
    let t27894 = t675 * t3;
    let t27895 = t3141 * t27894;
    let t27905 = -5.0 / 144.0 * t20171 + t8526 * t10195 * t8514 / 8.0 + t8526 * t3140 * t27852 / 8.0 - t27857 / 72.0 - t3138 * t8502 * t10191 / 24.0 - t3138 * t3140 * t214 * t10325 * t675 / 24.0 - t3138 * t3140 * t10190 * t2002 / 48.0 - 7.0 / 144.0 * t8511 * t8513 * t27871 - t684 * t687 * t10158 * t2002 / 64.0 + t27880 / 144.0 + t684 * t3150 * t8561 * t3 / 8.0 - t684 * t687 * t6457 * t3938 / 64.0 - t684 * t687 * t2033 * t10325 / 32.0 - t8526 * t8519 * t27895 / 2.0 + 7.0 / 18.0 * t8511 * t23930 * t27895 + t23783 / 16.0 - t23788 / 96.0 - t23791 / 72.0;
    (t27894, t27905)
}
