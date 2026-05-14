//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1283/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1283<F: Float>(t2185: F, t2562: F, t2148: F, t24063: F, t6407: F, t7407: F, t1234: F, t921: F, t538: F, t6155: F, t2162: F, t2228: F, t7625: F, t6395: F, t8067: F, t2207: F, t2208: F, t8279: F) -> (F, F, F, F, F, F, F, F) {
    let t24064 = t2562 * t2185;
    let t24066 = t24063 * t2148 * t24064;
    let t24068 = t6407 * t7407;
    let t24070 = t921 * t1234;
    let t24072 = t6155 * t538 * t24070;
    let t24074 = t2228 * t2162;
    let t24075 = t24074 * t7625;
    let t24076 = 0.4939086887201633699e-1 * t24075;
    let t24077 = t6395 * t8067;
    let t24080 = t2207 * t8279 * t2208;
    (t24066, t24068, t24070, t24072, t24074, t24076, t24077, t24080)
}
