//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 673/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk673<F: Float>(t1061: F, t1923: F, t256: F, t1918: F, t2654: F, t1639: F, t649: F, t1642: F, t1: F, t837: F, t1033: F, t1778: F) -> (F, F, F, F, F, F, F) {
    let t7733 = t1061 * t1923;
    let t7734 = t7733 * t256;
    let t7736 = t2654 * t1918;
    let t7758 = t1639 * t649;
    let t7759 = t7758 * t1642;
    let t7776 = t1 * t837;
    let t7811 = t1033 * t1778;
    (t7733, t7734, t7736, t7758, t7759, t7776, t7811)
}
