//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 946/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk946<F: Float>(t1458: F, t640: F, t103: F, t4054: F, t1: F, t102: F, t1509: F, t681: F, t1689: F, t1302: F, t1457: F, t126: F, t1303: F, t1946: F, t1609: F, t1615: F) -> (F, F, F, F, F, F, F, F) {
    let t14940 = t1458 * t640;
    let t15260 = t4054 * t103;
    let t15284 = t681 * t1 * t102 * t1509;
    let t15341 = t1689 * t1509;
    let t15354 = t1302 * t1457;
    let t15355 = t15354 * t126;
    let t15358 = t1946 * t102 * t1303;
    let t15430 = t1609 * t1615;
    (t14940, t15260, t15284, t15341, t15354, t15355, t15358, t15430)
}
