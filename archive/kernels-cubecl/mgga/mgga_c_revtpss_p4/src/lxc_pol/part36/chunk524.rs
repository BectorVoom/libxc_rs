//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 524/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk524<F: Float>(t1340: F, t2496: F, t2626: F, t1412: F, t73: F, t1389: F, t1408: F, t2736: F, t1425: F, t560: F) -> (F, F, F, F, F, F) {
    let t4037 = F::cast_from(0.17315859105681463759e2_f64) * t1340 * t2496;
    let t4042 = F::cast_from(0.11696447245269292414e1_f64) * t1340 * t2626;
    let t4049 = t73 * t1412;
    let t4062 = t1408 * t1389;
    let t4064 = F::cast_from(0.25410001404642664112e-5_f64) * t2736 * t4062;
    let t4075 = F::cast_from(1.0_f64) / t1425 / t560;
    (t4037, t4042, t4049, t4062, t4064, t4075)
}
