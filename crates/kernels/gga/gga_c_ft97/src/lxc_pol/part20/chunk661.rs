//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 661/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk661<F: Float>(t14332: F, t14352: F, t661: F, t2330: F, t3826: F, t1136: F, t9511: F, t1273: F, t2961: F, t4381: F, t2956: F, t4375: F, t909: F, t332: F, t505: F, t4380: F) -> (F, F, F, F, F, F, F) {
    let t14353 = t14332 + t14352;
    let t14354 = t661 * t14353;
    let t14358 = t2330 * t3826;
    let t14361 = t9511 * t1136;
    let t14390 = t1273 * t2961;
    let t14391 = t14390 * t4381;
    let t14394 = t1273 * t2956;
    let t14395 = t14394 * t4381;
    let t14402 = t4375 * t909;
    let t14403 = t14402 * t4381;
    let t14408 = t332 * t505;
    let t14409 = t4380 * t14408;
    (t14354, t14358, t14361, t14391, t14395, t14403, t14409)
}
