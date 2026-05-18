//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 1022/1311 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk1022<F: Float>(t1180: F, t5541: F, t1648: F, t583: F, t14873: F, t169: F, t103: F, t172: F, t5698: F, t4048: F, t561: F, t1037: F, t1552: F) -> (F, F, F, F, F, F) {
    let t21053 = t5541 * t1180;
    let t21054 = t1648 * t583;
    let t21072 = t169 * t14873;
    let t21076 = t5698 * t172 * t103;
    let t21084 = t561 * t4048;
    let t21111 = t1037 * t1552;
    (t21053, t21054, t21072, t21076, t21084, t21111)
}
