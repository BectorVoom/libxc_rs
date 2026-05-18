//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1282/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1282<F: Float>(t13984: F, t56320: F, t13972: F, t15371: F, t12248: F, t13780: F, t13859: F, t3990: F, t9926: F, t11401: F, t15357: F, t15366: F) -> (F, F, F, F, F, F, F) {
    let t56321 = t56320 * t13984;
    let t56323 = t13972 * t15371;
    let t56333 = t13859 * t3990 * t13780 * t12248;
    let t56337 = t13859 * t3990 * t13780 * t9926;
    let t56341 = t13859 * t3990 * t13780 * t11401;
    let t56343 = t13972 * t15357;
    let t56349 = t13972 * t15366;
    (t56321, t56323, t56333, t56337, t56341, t56343, t56349)
}
