//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 656/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk656<F: Float>(t2371: F, t2409: F, t3959: F, t1173: F, t894: F, t3958: F, t867: F) -> (F, F, F, F) {
    let t3960 = t2409 * t2371;
    let t3961 = t3959 * t3960;
    let t3963 = t1173 * t894;
    let t3965 = t3958 * t867;
    (t3960, t3961, t3963, t3965)
}
