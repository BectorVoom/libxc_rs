//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 805/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk805<F: Float>(t13916: F, t13952: F, t224: F, t5558: F, t744: F, t1980: F, t6110: F, t124: F, t1390: F, t10928: F, t1434: F, t822: F) -> (F, F, F, F, F, F) {
    let t13953 = t13916 + t13952;
    let t13954 = t224 * t13953;
    let t14537 = t744 * t5558;
    let t15362 = t1980 * t6110;
    let t15481 = t124 * t1390;
    let t15498 = t822 * t10928 * t1434;
    (t13953, t13954, t14537, t15362, t15481, t15498)
}
