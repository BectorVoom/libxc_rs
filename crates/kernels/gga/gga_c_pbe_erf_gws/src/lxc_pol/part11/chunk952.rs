//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 952/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk952<F: Float>(t12822: F, t2612: F, t12767: F, t30630: F, t10629: F, t3407: F, t1017: F, t1820: F, t40558: F, t7703: F, t16490: F, t18149: F, t18155: F, t42943: F, t42948: F, t47293: F, t47297: F) -> (F, F, F, F, F) {
    let t47299 = 32.0 / 15.0 * t2612 * t12822;
    let t47301 = 32.0 / 15.0 * t30630 * t12767;
    let t47303 = 32.0 / 15.0 * t10629 * t3407;
    let t47307 = 32.0 / 5.0 * t1820 * t7703 * t40558 * t1017;
    let t47308 = t18149 + 4.0 / 3.0 * t42943 - t18155 + 0.24311111111111111111e0 * t42948 - t16490 + t47293 + t47297 + t47299 + t47301 - t47303 + t47307;
    (t47299, t47301, t47303, t47307, t47308)
}
