//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1283/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1283<F: Float>(t15377: F, t2397: F, t15182: F, t51666: F, t14733: F, t8690: F, t11407: F, t14797: F, t3989: F, t3990: F, t12237: F, t13780: F, t14637: F) -> (F, F, F, F, F) {
    let t56351 = t15377 * t2397;
    let t56357 = t51666 * t15182;
    let t56362 = t14733 * t8690;
    let t56366 = t3989 * t3990 * t14797 * t11407;
    let t56374 = t14637 * t3990 * t13780 * t12237;
    (t56351, t56357, t56362, t56366, t56374)
}
