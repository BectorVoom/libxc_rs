//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1113/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1113<F: Float>(t5065: F, t537: F, t2133: F, t2294: F, t6287: F, t2139: F, t6249: F, t6118: F, t6300: F, t6571: F, t2183: F, t6148: F, t6149: F, t6303: F, t6114: F, t6152: F) -> (F, F, F, F, F, F, F, F) {
    let t20001 = t537 * t5065;
    let t20021 = t2133 * t2294 * t6287;
    let t20024 = t2139 * t2294 * t6249;
    let t20035 = t6118 * t6300;
    let t20038 = t2133 * t2294 * t6571;
    let t20040 = t2183 * t6148;
    let t20043 = t6149 * t6303;
    let t20049 = t6152 * t6114;
    (t20001, t20021, t20024, t20035, t20038, t20040, t20043, t20049)
}
