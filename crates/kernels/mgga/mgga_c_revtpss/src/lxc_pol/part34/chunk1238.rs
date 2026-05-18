//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1238/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1238<F: Float>(t11108: F, t7840: F, t1711: F, t2411: F, t10309: F, t1470: F, t28126: F, t60224: F, t6957: F, t1513: F, t94975: F, t530: F, t7933: F) -> (F, F, F, F, F, F, F) {
    let t100806 = t7840 * t11108;
    let t100987 = t2411 * t1711;
    let t101252 = t10309 * t1470;
    let t101333 = t10309 * t28126;
    let t101342 = t60224 * t6957;
    let t101451 = t94975 * t1513;
    let t101473 = t530 * t7933;
    (t100806, t100987, t101252, t101333, t101342, t101451, t101473)
}
