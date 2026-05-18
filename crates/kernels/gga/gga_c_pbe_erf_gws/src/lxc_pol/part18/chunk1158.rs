//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1158/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1158<F: Float>(t12213: F, t2409: F, t4016: F, t4182: F, t938: F, t3067: F, t3111: F, t3950: F, t850: F, t833: F, t1123: F, t13815: F) -> (F, F, F, F, F, F) {
    let t14664 = t2409 * t12213 * t4016;
    let t14667 = t4182 * t938;
    let t14669 = t2409 * t3067 * t14667;
    let t14673 = t850 * t3111 * t3950;
    let t14674 = t14673 * t833;
    let t14677 = t850 * t1123 * t13815;
    (t14664, t14667, t14669, t14673, t14674, t14677)
}
