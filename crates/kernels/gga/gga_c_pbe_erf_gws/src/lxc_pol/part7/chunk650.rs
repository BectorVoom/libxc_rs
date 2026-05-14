//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 650/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk650<F: Float>(t2704: F, t2718: F, t248: F, t256: F, t1910: F, t723: F, t1924: F, t1917: F, t245: F, t712: F, t1903: F, t708: F, t1914: F, t5384: F, t5387: F, t5388: F, t5390: F, t5397: F, t5405: F, t5408: F, t5410: F, t5412: F, t5415: F, t5417: F, t5418: F, t5423: F) -> (F, F, F, F) {
    let t5426 = 0.10059259259259259259e0 * t2704 - 0.50074074074074074075e0 * t2718;
    let t5427 = t248 * t5426;
    let t5429 = t5427 * t256 / 3.0;
    let t5430 = t1910 * t723;
    let t5433 = 2.0 / 3.0 * t1924 * t723;
    let t5434 = t245 * t1917;
    let t5436 = 0.2e-20 * t712 * t5434;
    let t5437 = t708 * t1903;
    let t5439 = -t5384 + t5387 + 2.0 / 3.0 * t5388 + 0.2e-20 * t1914 * t5390 + t5397 + t5405 + t5408 + t5410 + t5412 + t5415 + t5417 + 0.36466666666666666665e0 * t5418 + t5423 + t5429 + 4.0 / 3.0 * t5430 + t5433 + t5436 - 2.0 / 9.0 * t5437;
    (t5426, t5427, t5434, t5439)
}
