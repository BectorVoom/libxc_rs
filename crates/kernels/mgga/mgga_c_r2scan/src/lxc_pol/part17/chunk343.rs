//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 343/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk343<F: Float>(t406: F, t425: F, t458: F, t99: F, t101: F, t445: F) -> (F, F, F, F, F, F) {
    let t1357 = t406 * t425;
    let t1359 = t406 * t458;
    let t1360 = F::cast_from(8.0_f64) * t1359;
    let t1361 = F::cast_from(1.0_f64) / t99;
    let t1368 = F::cast_from(1.0_f64) / t101;
    let t1379 = t445 * t445;
    let t1380 = F::cast_from(1.0_f64) / t1379;
    (t1357, t1360, t1361, t1368, t1379, t1380)
}
