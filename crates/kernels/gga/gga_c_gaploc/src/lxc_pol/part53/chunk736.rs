//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 736/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk736<F: Float>(t14521: F, t224: F, t5558: F, t744: F, t1980: F, t6110: F, t124: F, t1390: F, t10928: F, t1434: F, t822: F, t169: F, t5750: F) -> (F, F, F, F, F, F) {
    let t14522 = t224 * t14521;
    let t14537 = t744 * t5558;
    let t15362 = t1980 * t6110;
    let t15481 = t124 * t1390;
    let t15498 = t822 * t10928 * t1434;
    let t15499 = t169 * t5750;
    (t14522, t14537, t15362, t15481, t15498, t15499)
}
