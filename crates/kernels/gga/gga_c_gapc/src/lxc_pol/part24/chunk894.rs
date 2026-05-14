//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 894/1133 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk894<F: Float>(t1456: F, t4046: F, t115: F, t126: F, t172: F, t442: F, t102: F, t1403: F, t1593: F, t1458: F, t640: F, t103: F, t4054: F, t1: F, t1509: F, t681: F) -> (F, F, F, F, F, F, F) {
    let t14873 = 1.0 / t4046 / t1456;
    let t14875 = t115 * t14873 * t126;
    let t14880 = t172 * M_PI * t442;
    let t14891 = t1593 * t102 * t1403;
    let t14940 = t1458 * t640;
    let t15260 = t4054 * t103;
    let t15284 = t681 * t1 * t102 * t1509;
    (t14873, t14875, t14880, t14891, t14940, t15260, t15284)
}
