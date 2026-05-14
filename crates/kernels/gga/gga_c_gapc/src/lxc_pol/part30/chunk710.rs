//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 710/1135 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk710<F: Float>(t1510: F, t1720: F, t3108: F, t1043: F, t1668: F, t3017: F, t5022: F, t3157: F, t8948: F, t1645: F, t190: F, t1649: F, t1643: F, t3171: F, t507: F, t3170: F) -> (F, F, F, F, F, F, F, F) {
    let t9155 = t1720 * t1510;
    let t9156 = t3108 * t9155;
    let t9158 = t1043 * t1668;
    let t9160 = t3017 * t5022;
    let t9161 = t1043 * t9160;
    let t9163 = t8948 * t3157;
    let t9166 = t190 * t1645 * M_PI;
    let t9167 = t9166 * t1649;
    let t9168 = t1643 * t9167;
    let t9173 = t3171 * t507;
    let t9174 = t3170 * t9173;
    (t9156, t9158, t9160, t9161, t9163, t9166, t9168, t9174)
}
