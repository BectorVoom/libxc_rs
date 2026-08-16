//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1052/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1052<F: Float>(t4447: F, t7561: F, t4384: F, t8511: F, t30371: F, t4376: F, t4380: F, t2068: F, t7422: F, t8480: F, t2264: F, t30456: F) -> (F, F, F, F, F, F) {
    let t34457 = t7561 * t4447;
    let t34459 = t8511 * t4384;
    let t34461 = t30371 * t4376;
    let t34463 = t8511 * t4380;
    let t34466 = t2068 * t8480 * t7422;
    let t34468 = t30456 * t2264;
    (t34457, t34459, t34461, t34463, t34466, t34468)
}
