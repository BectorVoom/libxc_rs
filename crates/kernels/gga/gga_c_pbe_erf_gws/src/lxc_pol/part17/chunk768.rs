//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 768/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk768<F: Float>(t329: F, t6594: F, t378: F, t2271: F, t2365: F, t822: F, t833: F, t2367: F, t2397: F, t745: F, t829: F, t830: F, t831: F, t891: F, t898: F) -> (F, F, F, F, F, F, F, F) {
    let t6729 = t329 * t6594;
    let t6731 = 455.0 / 1296.0 * t6729 * t378;
    let t6744 = t2271 * t2365;
    let t6745 = t822 * t6744;
    let t6746 = t6745 * t833;
    let t6748 = t2367 * t2397;
    let t6778 = t829 * t830 * t831 * t745;
    let t6781 = t891 * t898;
    (t6729, t6731, t6744, t6745, t6746, t6748, t6778, t6781)
}
