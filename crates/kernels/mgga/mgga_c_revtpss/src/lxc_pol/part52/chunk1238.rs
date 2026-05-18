//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1238/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1238<F: Float>(t111018: F, t1936: F, t28653: F, t7002: F, t1518: F, t7356: F, t2051: F, t4292: F, t34251: F, t26399: F, t7741: F, t28658: F) -> (F, F, F, F, F, F, F, F, F) {
    let t128332 = t111018 * t1936;
    let t128333 = t28653 * t7002;
    let t128334 = t7356 * t1518;
    let t128335 = t128334 * t1936;
    let t128336 = t2051 * t4292;
    let t128337 = t128336 * t1936;
    let t128338 = t34251 * t7002;
    let t128339 = t26399 * t7741;
    let t128340 = t28658 * t7741;
    (t128332, t128333, t128334, t128335, t128336, t128337, t128338, t128339, t128340)
}
