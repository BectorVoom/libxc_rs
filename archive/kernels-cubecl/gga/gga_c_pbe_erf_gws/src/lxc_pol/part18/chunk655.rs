//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 655/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk655<F: Float>(t142: F, t981: F, t2031: F, t159: F, t285: F, t3379: F, t1523: F, t3342: F, t3346: F, t476: F, t1528: F, t3351: F) -> (F, F, F, F, F, F) {
    let t3619 = t142 * t981;
    let t3620 = t2031 * t3619;
    let t3626 = t3379 * t159 * t285;
    let t3629 = t1523 * t3342;
    let t3631 = t476 * t3346;
    let t3633 = t1528 * t3351;
    (t3619, t3620, t3626, t3629, t3631, t3633)
}
