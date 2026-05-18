//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1231/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1231<F: Float>(t13780: F, t13859: F, t3990: F, t8754: F, t9326: F, t14705: F, t51666: F, t14637: F, t3974: F, t8759: F, t14633: F, t9213: F) -> (F, F, F, F, F, F) {
    let t53170 = t13859 * t3990 * t13780 * t8754;
    let t53174 = t13859 * t3990 * t13780 * t9326;
    let t53178 = t51666 * t14705;
    let t53182 = t14637 * t3990 * t3974 * t8759;
    let t53198 = t51666 * t14633;
    let t53207 = t14637 * t3990 * t13780 * t9213;
    (t53170, t53174, t53178, t53182, t53198, t53207)
}
