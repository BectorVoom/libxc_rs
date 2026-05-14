//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1078/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1078<F: Float>(t3985: F, t51529: F, t2079: F, t376: F, t361: F, t14797: F, t3973: F, t2299: F, t254: F, t3970: F, t13807: F, t13916: F, t2306: F, t332: F, t1477: F, t326: F) -> (F, F, F, F, F, F, F, F) {
    let t51530 = t51529 * t3985;
    let t51543 = t376 * t2079;
    let t51544 = t361 * t51543;
    let t51548 = t3973 * t14797;
    let t51555 = t3970 * t2299 * t254;
    let t51563 = t13807 * t13916;
    let t51580 = t2306 * t332;
    let t51649 = t326 * t1477;
    (t51530, t51543, t51544, t51548, t51555, t51563, t51580, t51649)
}
