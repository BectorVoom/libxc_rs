//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 657/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk657<F: Float>(t1340: F, t2626: F, t1412: F, t73: F, t1389: F, t1408: F, t2736: F, t1419: F, t213: F, t1425: F, t560: F, t225: F, t1429: F, t2435: F, t1428: F, t2777: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4042 = 0.11696447245269292414e1 * t1340 * t2626;
    let t4049 = t73 * t1412;
    let t4062 = t1408 * t1389;
    let t4064 = 0.25410001404642664112e-5 * t2736 * t4062;
    let t4071 = t213 * t1419;
    let t4075 = 1.0 / t1425 / t560;
    let t4076 = t225 * t4075;
    let t4082 = 0.73171657588172351096e-2 * t2435 * t1429;
    let t4083 = t2777 * t1428;
    (t4042, t4049, t4062, t4064, t4071, t4075, t4076, t4082, t4083)
}
