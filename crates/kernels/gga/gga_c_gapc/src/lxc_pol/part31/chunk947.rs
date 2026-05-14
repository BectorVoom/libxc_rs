//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 947/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk947<F: Float>(t1037: F, t1403: F, t102: F, t1338: F, t1946: F, t4864: F, t640: F, t11589: F, t567: F, t4: F, t4054: F, t4048: F, t431: F, t122: F, t457: F, t4882: F) -> (F, F, F, F, F, F, F, F, F) {
    let t13483 = t1037 * t1403;
    let t13537 = t1946 * t102 * t1338;
    let t13541 = t4864 * t640;
    let t13646 = t11589 * M_PI * t567;
    let t13654 = t4054 * t4;
    let t13675 = t431 * t4048;
    let t13676 = t13675 * t122;
    let t13679 = t4054 * M_PI * t457;
    let t13736 = t4882 * t122;
    (t13483, t13537, t13541, t13646, t13654, t13675, t13676, t13679, t13736)
}
