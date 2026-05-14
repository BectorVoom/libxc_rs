//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 807/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk807<F: Float>(t3228: F, t6402: F, t6365: F, t904: F, t2083: F, t3037: F, t3165: F, t6: F, t1112: F, t2079: F, t2319: F, t3299: F, t367: F, t6553: F, t899: F, t4394: F) -> (F, F, F, F, F, F, F, F, F) {
    let t9342 = 7.0 / 576.0 * t6402 * t3228;
    let t9343 = t6365 * t904;
    let t9364 = t3037 * t2083;
    let t9375 = t6 * t3165;
    let t9385 = t2079 * t1112;
    let t9386 = t904 * t9385;
    let t9415 = 7.0 / 1152.0 * t2319 * t3299;
    let t9425 = t899 * t6553 * t367;
    let t9441 = t1112 * t4394;
    (t9342, t9343, t9364, t9375, t9385, t9386, t9415, t9425, t9441)
}
