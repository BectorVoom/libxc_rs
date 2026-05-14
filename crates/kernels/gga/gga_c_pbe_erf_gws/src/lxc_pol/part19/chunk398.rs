//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 398/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk398<F: Float>(t1351: F, t1336: F, t169: F, t700: F, t770: F, t6: F, t837: F) -> (F, F, F, F) {
    let t1352 = 8.0 * t1351;
    let t1353 = 6.0 * t1336;
    let t1360 = t169 * t770 * t700;
    let t1365 = t6 * t837;
    (t1352, t1353, t1360, t1365)
}
