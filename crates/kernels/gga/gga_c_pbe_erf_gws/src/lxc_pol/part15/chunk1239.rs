//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1239/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1239<F: Float>(t1178: F, t51543: F, t50998: F, t9516: F, t2079: F, t898: F, t13917: F, t3258: F, t816: F, t820: F, t938: F, t13780: F, t13859: F, t3990: F, t8754: F) -> (F, F, F, F) {
    let t53156 = t1178 * t51543;
    let t53158 = t50998 * t53156 * t9516;
    let t53161 = t1178 * t898 * t2079;
    let t53166 = t13917 * t53161 * t3258 * t816 * t938 * t820;
    let t53170 = t13859 * t3990 * t13780 * t8754;
    (t53156, t53158, t53166, t53170)
}
