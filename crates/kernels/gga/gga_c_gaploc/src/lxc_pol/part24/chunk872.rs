//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 872/1270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk872<F: Float>(t3351: F, t484: F, t2854: F, t6509: F, t6320: F, t2268: F, t10150: F, t10153: F, t10157: F, t10162: F, t10165: F, t10169: F, t10172: F, t1358: F, t9089: F, t9092: F, t9094: F, t9147: F, t9149: F) -> (F, F, F) {
    let t10175 = t484 * t3351;
    let t10176 = 0.15808337019820083111e-2 * t10175;
    let t10177 = t2854 * t6509;
    let t10178 = t6320 * t10177;
    let t10180 = 0.17073003981405689759e0 * t2268 * t10178;
    let t10181 = t10150 + 0.56910013271352299198e-1 * t2268 * t10153 - 0.85365019907028448797e-1 * t2268 * t10157 - t10162 + t10165 - t10169 - 0.31616674039640166221e-2 * t1358 * t10172 - t9089 + t9092 - t9094 + t10176 - t10180 + t9147 - t9149;
    (t10177, t10178, t10181)
}
