//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1203/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1203<F: Float>(t2299: F, t254: F, t3970: F, t13925: F, t19777: F, t13807: F, t13916: F, t13920: F, t14791: F, t2417: F, t353: F, t859: F) -> (F, F, F, F, F) {
    let t51555 = t3970 * t2299 * t254;
    let t51561 = t19777 * t13925;
    let t51563 = t13807 * t13916;
    let t51564 = t51563 * t13920;
    let t51569 = t859 * t353 * t14791 * t2417;
    (t51555, t51561, t51563, t51564, t51569)
}
