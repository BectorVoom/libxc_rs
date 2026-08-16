//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1346/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1346<F: Float>(t11673: F, t14007: F, t3065: F, t3831: F, t2134: F, t14079: F, t3827: F, t14547: F, t38133: F, t6523: F, t3837: F, t51301: F) -> (F, F, F, F, F) {
    let t57171 = t14007 * t11673;
    let t57173 = t3065 * t3831;
    let t57174 = t2134 * t57173;
    let t57176 = t14079 * t3827;
    let t57179 = t14547 * t6523 * t38133;
    let t57182 = t51301 * t3837;
    (t57171, t57174, t57176, t57179, t57182)
}
