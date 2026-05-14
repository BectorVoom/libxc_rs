//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 602/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk602<F: Float>(t24: F, t5420: F, t712: F, t2704: F, t2718: F, t248: F, t256: F, t1924: F, t723: F, t1917: F, t245: F, t703: F, t713: F, t155: F, t641: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t5421 = t24 * t5420;
    let t5423 = 0.18233333333333333333e0 * t712 * t5421;
    let t5426 = 0.10059259259259259259e0 * t2704 - 0.50074074074074074075e0 * t2718;
    let t5427 = t248 * t5426;
    let t5429 = t5427 * t256 / 3.0;
    let t5433 = 2.0 / 3.0 * t1924 * t723;
    let t5434 = t245 * t1917;
    let t5436 = 0.2e-20 * t712 * t5434;
    let t5441 = t703 * t713;
    let t5443 = 0.13506172839506172839e-1 * t712 * t5441;
    let t5463 = t155 * t641;
    (t5421, t5423, t5426, t5427, t5429, t5433, t5434, t5436, t5441, t5443, t5463)
}
