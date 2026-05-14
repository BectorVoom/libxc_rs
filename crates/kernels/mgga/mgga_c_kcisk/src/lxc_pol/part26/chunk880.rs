//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 880/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk880<F: Float>(t20160: F, t6206: F, t1309: F, t2160: F, t3981: F, t25: F, t6212: F, t3970: F, t6196: F, t164: F, t2169: F, t3966: F, t1305: F, t6152: F, t3984: F, t6157: F) -> (F, F, F, F, F, F, F, F) {
    let t20161 = t20160 * t6206;
    let t20162 = t1309 * t20161;
    let t20169 = t2160 * t3981;
    let t20175 = t25 * t6212;
    let t20177 = 0.35981577432354634426e-1 * t1309 * t20175;
    let t20182 = t3970 * t6196;
    let t20184 = t164 * t2169;
    let t20185 = t1309 * t20184;
    let t20202 = 0.35981577432354634426e-1 * t3966 * t6196;
    let t20206 = t6152 * t1305;
    let t20226 = 0.35981577432354634426e-1 * t6157 * t3984;
    (t20162, t20169, t20177, t20182, t20185, t20202, t20206, t20226)
}
