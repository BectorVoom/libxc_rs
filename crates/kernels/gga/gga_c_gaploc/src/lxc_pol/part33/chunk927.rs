//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 927/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk927<F: Float>(t10166: F, t2326: F, t9074: F, t3351: F, t484: F, t2854: F, t6509: F, t6320: F, t2268: F, t3327: F, t6305: F, t4261: F, t7893: F) -> (F, F, F, F, F, F, F, F) {
    let t10167 = t10166 * t2326;
    let t10168 = t9074 * t10167;
    let t10169 = F::cast_from(0.35568758294595186999e-2_f64) * t10168;
    let t10175 = t484 * t3351;
    let t10176 = F::cast_from(0.15808337019820083111e-2_f64) * t10175;
    let t10177 = t2854 * t6509;
    let t10178 = t6320 * t10177;
    let t10180 = F::cast_from(0.17073003981405689759e0_f64) * t2268 * t10178;
    let t10184 = F::cast_from(0.28455006635676149599e-1_f64) * t6305 * t3327;
    let t10185 = t4261 * t7893;
    (t10167, t10169, t10176, t10177, t10178, t10180, t10184, t10185)
}
