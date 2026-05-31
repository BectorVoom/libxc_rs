//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 1074/1444 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk1074<F: Float>(t101: F, t1645: F, t1456: F, t4046: F, t115: F, t126: F, t172: F, t442: F, t102: F, t1403: F, t1593: F, t1458: F, t640: F) -> (F, F, F, F, F, F) {
    let pi = F::cast_from(M_PI);
    let t14541 = t1645 * t101;
    let t14873 = F::cast_from(1.0_f64) / t4046 / t1456;
    let t14875 = t115 * t14873 * t126;
    let t14880 = t172 * pi * t442;
    let t14891 = t1593 * t102 * t1403;
    let t14940 = t1458 * t640;
    (t14541, t14873, t14875, t14880, t14891, t14940)
}
