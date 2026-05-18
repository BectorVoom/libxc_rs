//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 768/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk768<F: Float>(t1043: F, t1668: F, t3017: F, t5022: F, t3157: F, t8948: F, t1645: F, t190: F, t1649: F, t1643: F, t3171: F, t507: F) -> (F, F, F, F, F, F, F) {
    let t9158 = t1043 * t1668;
    let t9160 = t3017 * t5022;
    let t9161 = t1043 * t9160;
    let t9163 = t8948 * t3157;
    let t9166 = t190 * t1645 * M_PI;
    let t9167 = t9166 * t1649;
    let t9168 = t1643 * t9167;
    let t9173 = t3171 * t507;
    (t9158, t9160, t9161, t9163, t9166, t9168, t9173)
}
