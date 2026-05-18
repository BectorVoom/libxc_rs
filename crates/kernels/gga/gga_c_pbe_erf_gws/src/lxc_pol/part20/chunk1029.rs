//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1029/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1029<F: Float>(t11514: F, t2171: F, t2345: F, t6229: F, t11464: F, t3140: F, t3235: F, t3752: F, t810: F, t1123: F, t2255: F, t814: F) -> (F, F, F, F, F, F) {
    let t11516 = t2345 * t11514 * t2171;
    let t11519 = F::new(35.0) / F::new(432.0) * t6229;
    let t11521 = t3235 * t11464 * t3140;
    let t11524 = t3752 * t810;
    let t11525 = t1123 * t11524;
    let t11526 = t2255 * t11525;
    let t11529 = t3752 * t814;
    (t11516, t11519, t11521, t11525, t11526, t11529)
}
