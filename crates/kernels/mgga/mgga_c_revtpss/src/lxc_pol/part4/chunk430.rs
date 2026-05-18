//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 430/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk430<F: Float>(t1399: F, t828: F, t1390: F, t550: F, t844: F, t247: F, t548: F, t235: F, t545: F) -> (F, F, F) {
    let t1400 = t828 * t1399;
    let t1401 = t1390 * t1400;
    let t1404 = t844 * t550;
    let t1405 = t1404 * t247;
    let t1407 = F::new(0.10003937560882938627e-2) * t548 * t1405;
    let t1408 = t545 * t235;
    (t1401, t1407, t1408)
}
