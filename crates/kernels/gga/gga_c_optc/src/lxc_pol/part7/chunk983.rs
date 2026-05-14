//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 983/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk983<F: Float>(t7119: F, t9917: F, t2035: F, t21933: F, t6778: F, t7110: F, t7122: F, t7133: F, t6928: F, t115: F, t658: F, t5: F, t151: F, t2124: F, t2126: F, t2168: F, t22161: F, t22168: F, t22172: F, t22178: F, t22187: F, t22192: F, t22224: F, t22883: F, t3467: F, t6931: F, t7129: F, t9961: F) -> (F,) {
    let t23234 = t9917 * t7119;
    let t23247 = t21933 * t2035;
    let t23254 = t7110 * t6778;
    let t23259 = t7122 * t7133;
    let t23267 = t7110 * t6928;
    let t23269 = t658 * t115;
    let t23270 = t23269 * t5;
    let t23274 = -0.48681704342817043984e1 * t23234 + 0.24182738140014814697e0 * t2168 * t22161 + 0.417271751509860377e1 * t9961 * t2126 * t22187 - 0.31295381363239528276e1 * t2124 * t7129 * t22172 + 0.10431793787746509425e1 * t2124 * t2126 * t22224 - 0.36274107210022222046e1 * t2168 * t6931 * t23247 - 0.62590762726479056552e1 * t2124 * t7129 * t23247 + 0.8463958349005185144e0 * t23254 - 0.31295381363239528276e1 * t2124 * t7129 * t22178 - 0.48681704342817043985e1 * t23259 + 0.31295381363239528276e1 * t3467 * t151 * t22883 - 0.31295381363239528276e1 * t9961 * t151 * t22192 - 0.33855833396020740576e1 * t23267 + 0.83454350301972075403e1 * t2124 * t23270 * t22168;
    (t23274,)
}
