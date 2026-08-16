//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2307/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2307<F: Float>(t21013: F, t3782: F, t12712: F, t471: F, t1774: F, t3367: F, t17934: F, t5330: F, t1248: F, t3604: F, t3670: F, t5390: F) -> (F, F, F, F, F, F) {
    let t21017 = t3782 * t21013;
    let t21028 = t12712 * t471;
    let t21035 = t1774 * t3367;
    let t21049 = t17934 * t5330;
    let t21119 = t3604 * t1248;
    let t21203 = t3670 * t5390;
    (t21017, t21028, t21035, t21049, t21119, t21203)
}
