//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 952/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk952<F: Float>(t861: F, t867: F, t786: F, t2467: F, t2828: F, t676: F, t123: F, t2465: F, t2410: F, t261: F, t2832: F, t892: F, t2408: F, t2411: F, t3335: F, t389: F) -> (F, F, F, F, F, F, F) {
    let t11043 = t861 * t867;
    let t11044 = t786 * t11043;
    let t11045 = t11044 * t2467;
    let t11049 = t676 * t2828;
    let t11050 = t123 * t11049;
    let t11051 = t2465 * t11050;
    let t11064 = 1.0 / t2410 / t261;
    let t11075 = t2832 * t892;
    let t11084 = t2408 * t2411;
    let t11108 = 1.0 / t3335 / t389;
    (t11044, t11045, t11051, t11064, t11075, t11084, t11108)
}
