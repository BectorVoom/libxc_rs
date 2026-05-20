//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2568/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2568<F: Float>(t2482: F, t4000: F, t596: F, t10003: F, t1412: F, t3923: F, t2661: F, t9835: F, t9934: F, t9914: F, t9918: F, t221: F, t4018: F, t4019: F, t9899: F) -> (F, F, F, F, F) {
    let t47215 = t2482 * t4000 * t596;
    let t47216 = t47215 * t10003;
    let t47218 = t1412 * t3923;
    let t47221 = t2661 * t9934 * t47218 * t9835;
    let t47223 = t9918 * t9914;
    let t47227 = t4018 * t4019 * t221 * t9899;
    (t47216, t47218, t47221, t47223, t47227)
}
