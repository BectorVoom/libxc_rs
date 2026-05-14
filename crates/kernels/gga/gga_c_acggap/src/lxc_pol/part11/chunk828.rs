//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 828/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk828<F: Float>(t1988: F, t7535: F, t30589: F, t7548: F, t2109: F, t7630: F, t2113: F, t30546: F, t7499: F, t30543: F, t7867: F, t2450: F, t7432: F) -> (F, F, F, F, F, F, F) {
    let t30918 = t1988 * t7535;
    let t30920 = t30589 * t7548;
    let t30921 = 0.41930789719472202756e-2 * t30920;
    let t30924 = t7630 * t2109;
    let t30926 = t7630 * t2113;
    let t30928 = t30546 * t7499;
    let t30932 = t30543 * t7867;
    let t30934 = t2450 * t7432;
    (t30918, t30921, t30924, t30926, t30928, t30932, t30934)
}
