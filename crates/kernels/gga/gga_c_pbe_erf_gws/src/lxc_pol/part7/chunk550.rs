//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 550/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk550<F: Float>(t140: F, t1503: F, t119: F, t132: F, t506: F, t9: F, t332: F, t857: F) -> (F, F, F, F) {
    let t2857 = t1503 * t140;
    let t2911 = t132 * t119;
    let t2912 = t9 * t506;
    let t3065 = t332 * t857;
    (t2857, t2911, t2912, t3065)
}
