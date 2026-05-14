//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 933/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk933<F: Float>(t2083: F, t3037: F, t3259: F, t3257: F, t2084: F, t816: F, t2079: F, t343: F, t3220: F, t3165: F, t6: F, t254: F, t3223: F, t2113: F, t274: F, t3221: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t9364 = t3037 * t2083;
    let t9365 = t9364 * t3259;
    let t9366 = t3257 * t9365;
    let t9370 = t2084 * t816;
    let t9371 = t343 * t2079 * t9370;
    let t9372 = t3220 * t9371;
    let t9375 = t6 * t3165;
    let t9376 = t254 * t9375;
    let t9377 = t9376 * t3223;
    let t9380 = t2113 * t274;
    let t9381 = t3221 * t9380;
    (t9364, t9365, t9366, t9370, t9371, t9372, t9375, t9377, t9380, t9381)
}
