//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 672/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk672<F: Float>(t28023: F, t7290: F, t1890: F, t28013: F, t28236: F, t739: F, t10036: F, t2021: F, t1980: F, t9816: F, t501: F, t9241: F, t5538: F, t883: F, t28668: F, t2547: F, t279: F, t481: F) -> (F, F, F, F, F, F, F, F, F) {
    let t28924 = t7290 * t28023;
    let t28953 = t1890 * t28013;
    let t28957 = t739 * t28236;
    let t28973 = t2021 * t10036;
    let t28983 = t1980 * t9816;
    let t29096 = t9241 * t501;
    let t29277 = t5538 * t883;
    let t29285 = t7290 * t28668;
    let t29439 = t481 * t2547 * t279;
    (t28924, t28953, t28957, t28973, t28983, t29096, t29277, t29285, t29439)
}
