//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1035/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1035<F: Float>(t11609: F, t2306: F, t3074: F, t860: F, t11473: F, t2345: F, t3219: F, t3717: F, t6: F, t875: F, t3757: F, t814: F) -> (F, F, F, F, F) {
    let t11610 = t2306 * t11609;
    let t11611 = t3074 * t11610;
    let t11613 = t11611 * t860 / F::new(96.0);
    let t11615 = t2345 * t3219 * t11473;
    let t11618 = t6 * t3717;
    let t11620 = t2345 * t11618 * t875;
    let t11623 = t3757 * t814;
    (t11613, t11615, t11618, t11620, t11623)
}
