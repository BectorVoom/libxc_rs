//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1288/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1288<F: Float>(t12248: F, t13780: F, t13859: F, t3990: F, t9926: F, t11401: F, t13972: F, t15357: F, t15366: F, t15377: F, t2397: F, t15182: F, t51666: F) -> (F, F, F, F, F, F, F) {
    let t56333 = t13859 * t3990 * t13780 * t12248;
    let t56337 = t13859 * t3990 * t13780 * t9926;
    let t56341 = t13859 * t3990 * t13780 * t11401;
    let t56343 = t13972 * t15357;
    let t56349 = t13972 * t15366;
    let t56351 = t15377 * t2397;
    let t56357 = t51666 * t15182;
    (t56333, t56337, t56341, t56343, t56349, t56351, t56357)
}
