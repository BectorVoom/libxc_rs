//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 497/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk497<F: Float>(t3356: F, t406: F, t281: F, t2902: F, t414: F, t1146: F, t698: F, t1224: F, t240: F, t1129: F, t408: F, t421: F) -> (F, F, F, F, F, F, F, F) {
    let t3402 = F::new(0.39862222222222222223e0) * t3356;
    let t3407 = F::new(1.0)/f64::sqrt(t406);
    let t3413 = t281 * t2902 * t414;
    let t3414 = F::new(0.13692777777777777778e0) * t3413;
    let t3415 = t698 * t1146;
    let t3417 = t240 * t1224;
    let t3431 = t1129 * t1129;
    let t3432 = F::new(1.0) / t3431;
    let t3433 = t408 * t3432;
    let t3434 = t421 * t421;
    (t3402, t3407, t3413, t3414, t3415, t3417, t3433, t3434)
}
