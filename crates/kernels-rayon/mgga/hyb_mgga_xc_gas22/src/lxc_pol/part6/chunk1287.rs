//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1287/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1287(t10190: f64, t2028: f64, t3934: f64, t6469: f64, t684: f64, t3: f64, t675: f64, t3141: f64, t10158: f64, t10191: f64, t10195: f64, t10325: f64, t2002: f64, t20171: f64, t2033: f64, t214: f64, t23783: f64, t23788: f64, t23791: f64, t23930: f64, t27852: f64, t27857: f64, t3138: f64, t3140: f64, t3150: f64, t3938: f64, t6457: f64, t687: f64, t8502: f64, t8511: f64, t8513: f64, t8514: f64, t8519: f64, t8526: f64, t8561: f64) -> (f64, f64, f64) {
    let t27871 = t10190 * t2028;
    let t27880 = t684 * t6469 * t3934;
    let t27894 = t675 * t3;
    let t27895 = t3141 * t27894;
    let t27905 = -5.0_f64 / 144.0_f64 * t20171 + t8526 * t10195 * t8514 / 8.0_f64 + t8526 * t3140 * t27852 / 8.0_f64 - t27857 / 72.0_f64 - t3138 * t8502 * t10191 / 24.0_f64 - t3138 * t3140 * t214 * t10325 * t675 / 24.0_f64 - t3138 * t3140 * t10190 * t2002 / 48.0_f64 - 7.0_f64 / 144.0_f64 * t8511 * t8513 * t27871 - t684 * t687 * t10158 * t2002 / 64.0_f64 + t27880 / 144.0_f64 + t684 * t3150 * t8561 * t3 / 8.0_f64 - t684 * t687 * t6457 * t3938 / 64.0_f64 - t684 * t687 * t2033 * t10325 / 32.0_f64 - t8526 * t8519 * t27895 / 2.0_f64 + 7.0_f64 / 18.0_f64 * t8511 * t23930 * t27895 + t23783 / 16.0_f64 - t23788 / 96.0_f64 - t23791 / 72.0_f64;
    (t27871, t27894, t27905)
}
