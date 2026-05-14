//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 618/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk618<F: Float>(t1664: F, t3342: F, t1661: F, t587: F, t1017: F, t2635: F, t1885: F, t1820: F, t2807: F, t1714: F, t3465: F, t3469: F, t657: F, t3473: F, t1688: F, t1709: F, t25: F, t2696: F, t2710: F, t3467: F, t3471: F, t3475: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t3530 = t1664 * t3342;
    let t3531 = t1661 * t3530;
    let t3533 = 4.0 / 27.0 * t587 * t3531;
    let t3534 = t2635 * t1017;
    let t3535 = t1885 * t3534;
    let t3537 = 8.0 / 15.0 * t1820 * t3535;
    let t3538 = 8.0 / 45.0 * t2807;
    let t3544 = t1714 * t3465;
    let t3547 = t657 * t3469;
    let t3550 = t657 * t3473;
    let t3553 = t1688 + 0.23994444444444444444e-1 * t2696 - 0.23994444444444444445e-1 * t3467 + 0.71983333333333333334e-1 * t3471 - 0.35991666666666666667e-1 * t3475 + t1709 + 0.8888888888888888889e-2 * t2710 - 0.22222222222222222222e-2 * t25 * t3544 + 0.13333333333333333333e-1 * t25 * t3547 - 0.66666666666666666667e-2 * t25 * t3550;
    (t3530, t3531, t3533, t3534, t3535, t3537, t3538, t3544, t3547, t3550, t3553)
}
