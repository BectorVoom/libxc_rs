//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 955/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk955<F: Float>(t12723: F, t30876: F, t1820: F, t1821: F, t41787: F, t950: F, t10442: F, t3342: F, t587: F, t12464: F, t2559: F, t995: F, t10938: F, t1827: F, t3346: F, t32260: F, t5543: F) -> (F, F, F, F, F, F, F) {
    let t47343 = 32.0 / 15.0 * t30876 * t12723;
    let t47347 = 32.0 / 45.0 * t1820 * t1821 * t41787 * t950;
    let t47348 = t10442 * t3342;
    let t47351 = 16.0 / 5.0 * t587 * t1821 * t47348;
    let t47355 = 64.0 / 9.0 * t1820 * t2559 * t12464 * t995;
    let t47359 = 8.0 / 15.0 * t587 * t1827 * t10938 * t3346;
    let t47363 = 8.0 / 9.0 * t587 * t5543 * t32260 * t3342;
    (t47343, t47347, t47348, t47351, t47355, t47359, t47363)
}
