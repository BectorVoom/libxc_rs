//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 726/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk726<F: Float>(t1586: F, t465: F, t148: F, t519: F, t1503: F, t534: F, t471: F, t204: F, t492: F) -> (F, F, F, F, F, F) {
    let t5044 = t465 * t1586;
    let t5048 = t148 * t519;
    let t5052 = t465 * t1503;
    let t5056 = t148 * t534;
    let t5063 = t148 * t471;
    let t5066 = F::new(0.71233333333333333332e-1) * t204 * t5063 * t492;
    (t5044, t5048, t5052, t5056, t5063, t5066)
}
