//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 640/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk640<F: Float>(t1918: F, t2654: F, t1639: F, t649: F, t1642: F, t1: F, t837: F, t1033: F, t1778: F, t1045: F, t1672: F, t211: F, t219: F, t5400: F, t5480: F, t1663: F, t995: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t7736 = t2654 * t1918;
    let t7758 = t1639 * t649;
    let t7759 = t7758 * t1642;
    let t7776 = t1 * t837;
    let t7811 = t1033 * t1778;
    let t7844 = t1672 * t1045;
    let t7845 = t211 * t7844;
    let t7853 = t5400 * t219;
    let t7877 = t5480 * t219;
    let t7899 = t995 * t1663;
    (t7736, t7758, t7759, t7776, t7811, t7844, t7845, t7853, t7877, t7899)
}
