//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 670/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk670<F: Float>(t639: F, t7459: F, t219: F, t641: F, t1639: F, t1642: F, t5219: F, t995: F, t1697: F, t5212: F, t1802: F, t589: F) -> (F, F, F, F, F, F) {
    let t7460 = t639 * t7459;
    let t7483 = t641 * t219;
    let t7490 = t1639 * t219;
    let t7491 = t7490 * t1642;
    let t7495 = t5219 * t995;
    let t7505 = t5212 * t1697;
    let t7514 = t589 * t1802;
    (t7460, t7483, t7491, t7495, t7505, t7514)
}
