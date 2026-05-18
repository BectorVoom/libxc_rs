//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 973/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk973<F: Float>(t2391: F, t3039: F, t2246: F, t3090: F, t3094: F, t3309: F, t840: F, t3306: F, t938: F, t2409: F, t3067: F, t3075: F, t331: F) -> (F, F, F, F, F, F, F) {
    let t8634 = t3039 * t2391;
    let t8641 = F::new(7.0) / F::new(72.0) * t2246 * t3090;
    let t8643 = F::new(7.0) / F::new(72.0) * t2246 * t3094;
    let t8646 = F::new(7.0) / F::new(144.0) * t840 * t3309;
    let t8647 = t3306 * t938;
    let t8649 = t2409 * t3067 * t8647;
    let t8652 = t3075 * t331;
    (t8634, t8641, t8643, t8646, t8647, t8649, t8652)
}
