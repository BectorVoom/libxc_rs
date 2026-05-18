//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 1010/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk1010<F: Float>(t103: F, t1431: F, t1037: F, t1403: F, t102: F, t1338: F, t1946: F, t4864: F, t640: F, t11589: F, t567: F, t4: F, t4054: F) -> (F, F, F, F, F, F) {
    let t13337 = t103 * t1431;
    let t13483 = t1037 * t1403;
    let t13537 = t1946 * t102 * t1338;
    let t13541 = t4864 * t640;
    let t13646 = t11589 * M_PI * t567;
    let t13654 = t4054 * t4;
    (t13337, t13483, t13537, t13541, t13646, t13654)
}
