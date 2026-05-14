//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 595/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk595<F: Float>(t1936: F, t2322: F, t5523: F, t1312: F, t7002: F, t1315: F, t196: F, t197: F) -> (F, F, F, F) {
    let t7226 = 2.0 * t2322 * t1936;
    let t7228 = 2.0 * t5523 * t1936;
    let t7230 = 2.0 * t1312 * t7002;
    let t7234 = t1315 * t196;
    let t7235 = t7234 * t197;
    (t7226, t7228, t7230, t7235)
}
