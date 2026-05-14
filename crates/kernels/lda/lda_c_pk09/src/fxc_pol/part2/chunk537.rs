//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 537/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk537<F: Float>(t106: F, t4274: F, t1046: F, t568: F, t933: F, t1146: F, t91: F, t97: F, t115: F, t3397: F, t3409: F, t3332: F, t3339: F, t3330: F, t3444: F, t3453: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t4275 = t106 * t4274;
    let t4277 = t568 * t1046;
    let t4278 = t933 * t4277;
    let t4280 = t91 * t1146;
    let t4281 = t4280 * t97;
    let t4283 = 5.0 / 27.0 * t115 * t4281;
    let t4299 = 0.510767601706895 * t3397;
    let t4302 = 2.2984542076810275 * t3409;
    let t4303 = 0.20376679178011928 * t3332;
    let t4304 = 0.033961131963353215 * t3339;
    let t4313 = 0.15282509383508946 * t3330;
    let t4320 = 2.2984542076810275 * t3444;
    let t4322 = 6.12921122048274 * t3453;
    (t4275, t4277, t4278, t4280, t4281, t4283, t4299, t4302, t4303, t4304, t4313, t4320, t4322)
}
