//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 582/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk582<F: Float>(t115: F, t4281: F, t3397: F, t3409: F, t3332: F, t3339: F, t3330: F, t3444: F, t3453: F, t1039: F, t133: F, t131: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4283 = F::cast_from(5.0_f64) / F::cast_from(27.0_f64) * t115 * t4281;
    let t4299 = F::cast_from(0.510767601706895_f64) * t3397;
    let t4302 = F::cast_from(2.2984542076810275_f64) * t3409;
    let t4303 = F::cast_from(0.20376679178011928_f64) * t3332;
    let t4304 = F::cast_from(0.033961131963353215_f64) * t3339;
    let t4313 = F::cast_from(0.15282509383508946_f64) * t3330;
    let t4320 = F::cast_from(2.2984542076810275_f64) * t3444;
    let t4322 = F::cast_from(6.12921122048274_f64) * t3453;
    let t4334 = t133 * t1039;
    let t4335 = t131 * t4334;
    (t4283, t4299, t4302, t4303, t4304, t4313, t4320, t4322, t4335)
}
