//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 859/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk859<F: Float>(t3293: F, t3297: F, t869: F, t134: F, t2299: F, t941: F, t3405: F, t3403: F, t2639: F, t9832: F, t1069: F, t2795: F) -> (F, F, F, F, F) {
    let t9986 = t869 * t3293 * t3297;
    let t9988 = t134 * t2299;
    let t9989 = t941 * t9988;
    let t9990 = t3405 * t9989;
    let t9991 = t3403 * t9990;
    let t9993 = t9832 * t2639;
    let t9995 = t1069 * t2795;
    (t9986, t9990, t9991, t9993, t9995)
}
