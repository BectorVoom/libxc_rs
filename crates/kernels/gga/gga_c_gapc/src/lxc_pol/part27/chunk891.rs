//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 891/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk891<F: Float>(t1736: F, t474: F, t177: F, t208: F, t4913: F, t319: F, t337: F, t7061: F, t103: F, t1431: F, t1037: F, t1403: F, t102: F, t1338: F, t1946: F, t4864: F, t640: F) -> (F, F, F, F, F, F, F) {
    let t12768 = t474 * t1736;
    let t13281 = t177 / t4913 / t208;
    let t13296 = t319 / t7061 / t337;
    let t13337 = t103 * t1431;
    let t13483 = t1037 * t1403;
    let t13537 = t1946 * t102 * t1338;
    let t13541 = t4864 * t640;
    (t12768, t13281, t13296, t13337, t13483, t13537, t13541)
}
