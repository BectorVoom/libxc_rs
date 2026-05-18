//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 398/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk398<F: Float>(t1651: F, t592: F, t587: F, t1407: F, t591: F, t590: F, t187: F, t572: F) -> (F, F, F, F, F, F) {
    let t1652 = t1651 * t592;
    let t1653 = t587 * t1652;
    let t1654 = F::new(16.0) / F::new(135.0) * t1653;
    let t1655 = t591 * t1407;
    let t1656 = t590 * t1655;
    let t1658 = F::new(4.0) / F::new(45.0) * t587 * t1656;
    let t1660 = F::new(1.0) / t187 / t572;
    (t1652, t1654, t1655, t1656, t1658, t1660)
}
