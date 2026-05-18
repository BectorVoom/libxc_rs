//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1188/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1188<F: Float>(t13888: F, t2352: F, t353: F, t859: F, t1178: F, t13918: F, t13909: F, t892: F, t2416: F, t4052: F, t938: F, t13808: F, t13906: F) -> (F, F, F, F, F, F) {
    let t51063 = t859 * t353 * t13888 * t2352;
    let t51066 = t1178 * t13918;
    let t51081 = t859 * t892 * t13909;
    let t51084 = t2416 * t4052;
    let t51087 = t859 * t353 * t51084 * t938;
    let t51096 = t13808 * t13906;
    (t51063, t51066, t51081, t51084, t51087, t51096)
}
