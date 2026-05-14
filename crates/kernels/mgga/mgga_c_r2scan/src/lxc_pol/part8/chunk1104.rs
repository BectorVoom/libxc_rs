//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1104/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1104<F: Float>(t1415: F, t57: F, t110: F, t6188: F, t6072: F, t1610: F, t6263: F, t783: F, t2132: F, t6217: F, t265: F, t267: F, t6079: F) -> (F, F, F, F, F, F) {
    let t20094 = t1415 * t57;
    let t20096 = t6188 * t20094 * t110;
    let t20097 = t20096 * t6072;
    let t20107 = t783 * t1610 * t6263;
    let t20127 = t6217 * t2132;
    let t20137 = t6079 * t265 * t267;
    (t20094, t20096, t20097, t20107, t20127, t20137)
}
