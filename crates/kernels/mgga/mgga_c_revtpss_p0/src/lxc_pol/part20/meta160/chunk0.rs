//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 864/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk864<F: Float>(t1121: F, t1263: F, t3362: F, t3617: F, t1012: F, t1224: F, t3698: F, t3623: F, t4890: F) -> (F, F, F, F, F) {
    let t5296 = t1263 * t1121;
    let t5302 = t3617 * t3362;
    let t5308 = t1012 * t1224;
    let t5312 = t1012 * t3698;
    let t5330 = t3623 * t4890;
    (t5296, t5302, t5308, t5312, t5330)
}
