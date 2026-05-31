//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1287/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1287<F: Float>(t10190: F, t2028: F, t3934: F, t6469: F, t684: F, t3: F, t675: F, t3141: F, t10158: F, t10191: F, t10195: F, t10325: F, t2002: F, t20171: F, t2033: F, t214: F, t23783: F, t23788: F, t23791: F, t23930: F, t27852: F, t27857: F, t3138: F, t3140: F, t3150: F, t3938: F, t6457: F, t687: F, t8502: F, t8511: F, t8513: F, t8514: F, t8519: F, t8526: F, t8561: F) -> (F, F, F) {
    let t27871 = t10190 * t2028;
    let t27880 = t684 * t6469 * t3934;
    let t27894 = t675 * t3;
    let t27895 = t3141 * t27894;
    let t27905 = -F::cast_from(5.0_f64) / F::cast_from(144.0_f64) * t20171 + t8526 * t10195 * t8514 / F::cast_from(8.0_f64) + t8526 * t3140 * t27852 / F::cast_from(8.0_f64) - t27857 / F::cast_from(72.0_f64) - t3138 * t8502 * t10191 / F::cast_from(24.0_f64) - t3138 * t3140 * t214 * t10325 * t675 / F::cast_from(24.0_f64) - t3138 * t3140 * t10190 * t2002 / F::cast_from(48.0_f64) - F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t8511 * t8513 * t27871 - t684 * t687 * t10158 * t2002 / F::cast_from(64.0_f64) + t27880 / F::cast_from(144.0_f64) + t684 * t3150 * t8561 * t3 / F::cast_from(8.0_f64) - t684 * t687 * t6457 * t3938 / F::cast_from(64.0_f64) - t684 * t687 * t2033 * t10325 / F::cast_from(32.0_f64) - t8526 * t8519 * t27895 / F::cast_from(2.0_f64) + F::cast_from(7.0_f64) / F::cast_from(18.0_f64) * t8511 * t23930 * t27895 + t23783 / F::cast_from(16.0_f64) - t23788 / F::cast_from(96.0_f64) - t23791 / F::cast_from(72.0_f64);
    (t27871, t27894, t27905)
}
