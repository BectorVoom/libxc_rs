//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1335/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1335<F: Float>(t13796: F, t13859: F, t14423: F, t6220: F, t13815: F, t3111: F, t833: F, t850: F, t1123: F, t50906: F, t14677: F, t2397: F) -> (F, F, F, F) {
    let t54512 = t13859 * t13796 * t14423 * t6220;
    let t54519 = t850 * t3111 * t13815 * t833;
    let t54523 = t850 * t1123 * t50906 * t833;
    let t54529 = t14677 * t2397;
    (t54512, t54519, t54523, t54529)
}
