//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 681/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk681<F: Float>(t2306: F, t3808: F, t905: F, t3126: F, t1105: F, t343: F) -> (F, F, F, F) {
    let t3809 = t3808 * t2306;
    let t3810 = t905 * t3809;
    let t3813 = F::new(7.0) / F::new(144.0) * t3126;
    let t3814 = t343 * t1105;
    (t3809, t3810, t3813, t3814)
}
