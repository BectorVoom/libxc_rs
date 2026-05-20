//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2729/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2729<F: Float>(t2829: F, t4321: F, t689: F, t15054: F, t786: F, t789: F, t2465: F, t4480: F, t9288: F, t1569: F, t2769: F, t10997: F) -> (F, F, F, F) {
    let t50198 = t689 * t4321 * t2829;
    let t50201 = t786 * t15054 * t789;
    let t50205 = t2465 * t4480 * t9288;
    let t50208 = t786 * t1569 * t2769;
    let t50209 = t50208 * t10997;
    (t50198, t50201, t50205, t50209)
}
