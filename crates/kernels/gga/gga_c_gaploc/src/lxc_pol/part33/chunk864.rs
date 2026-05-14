//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 864/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk864<F: Float>(t10175: F, t2854: F, t6509: F, t6320: F, t2268: F, t3327: F, t6305: F, t4261: F, t7893: F, t9074: F, t2312: F, t3351: F, t7974: F, t894: F, t1063: F, t9097: F, t9100: F, t9108: F, t9111: F, t9113: F, t9115: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t10176 = 0.15808337019820083111e-2 * t10175;
    let t10177 = t2854 * t6509;
    let t10178 = t6320 * t10177;
    let t10180 = 0.17073003981405689759e0 * t2268 * t10178;
    let t10184 = 0.28455006635676149599e-1 * t6305 * t3327;
    let t10185 = t4261 * t7893;
    let t10186 = t9074 * t10185;
    let t10187 = 0.23712505529730124666e-2 * t10186;
    let t10194 = t2312 * t3351;
    let t10195 = 0.11856252764865062333e-2 * t10194;
    let t10196 = t894 * t7974;
    let t10198 = 0.28455006635676149599e-1 * t1063 * t10196;
    let t10205 = -21.0 / 256.0 * t9097 + 147.0 / 8192.0 * t9100 - 63.0 / 524288.0 * t9108 + 21.0 / 524288.0 * t9111 - 49.0 / 8192.0 * t9113 + 7.0 / 256.0 * t9115;
    (t10176, t10177, t10178, t10180, t10184, t10185, t10187, t10195, t10196, t10198, t10205)
}
