//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 316/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk316<F: Float>(t330: F, t421: F, t829: F, t1252: F, t420: F) -> (F, F, F, F, F) {
    let t1253 = t421 * t330;
    let t1254 = t1253 * t829;
    let t1255 = t1252 * t1254;
    let t1258 = t420 * t420;
    let t1259 = 1.0 / t1258;
    (t1253, t1254, t1255, t1258, t1259)
}
