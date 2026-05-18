//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1077/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1077<F: Float>(t1: F, t4049: F, t172: F, t5963: F, t101: F, t1645: F, t1456: F, t4046: F, t115: F, t126: F, t442: F, t102: F, t1403: F, t1593: F) -> (F, F, F, F, F, F, F) {
    let t13850 = t4049 * t1;
    let t13853 = t5963 * t172;
    let t14541 = t1645 * t101;
    let t14873 = F::new(1.0) / t4046 / t1456;
    let t14875 = t115 * t14873 * t126;
    let t14880 = t172 * M_PI * t442;
    let t14891 = t1593 * t102 * t1403;
    (t13850, t13853, t14541, t14873, t14875, t14880, t14891)
}
