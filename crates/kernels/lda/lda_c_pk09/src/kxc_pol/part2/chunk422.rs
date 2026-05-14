//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 422/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk422<F: Float>(t2314: F, t80: F, t1094: F, t1014: F, t1015: F, t1019: F, t1020: F, t2159: F, t2163: F, t2167: F, t2171: F, t2175: F, t2179: F, t83: F, t89: F, t1047: F, t1052: F, t106: F, t1065: F, t1076: F, t1101: F, t115: F, t2155: F, t2210: F, t2214: F, t2305: F, t2337: F, t2341: F, t2355: F, t90: F, t98: F, t993: F, t994: F) -> (F, F, F, F, F, F) {
    let t2362 = t2314 * t80;
    let t2363 = t2362 * t1094;
    let t2378 = t1014 + t1015 + 4.431130547644593 * t2159 + 4.431130547644593 * t2163 - 4.431130547644593 * t2167 + t1019 + t1020 + 0.2946275542389858 * t2171 + 0.2946275542389858 * t2175 - 0.2946275542389858 * t2179;
    let t2379 = t83 * t2378;
    let t2380 = t2379 * t89;
    let t2385 = 0.14975624337724558 * t2155 + t2337 * t98 / 6.0 + t115 * t2341 / 6.0 + t2305 * t1047 / 36.0 - t993 - t994 + t1052 * t2210 / 6.0 + t1052 * t2214 / 6.0 - t2355 * t98 / 6.0 + t1076 * t2210 / 6.0 + t1076 * t2214 / 6.0 - t2363 * t98 / 6.0 - t1101 * t2210 / 6.0 - t1101 * t2214 / 6.0 - t106 * t2341 / 6.0 - t2380 * t98 / 6.0 + t1065 - t90 * t2341 / 6.0;
    (t2362, t2363, t2378, t2379, t2380, t2385)
}
