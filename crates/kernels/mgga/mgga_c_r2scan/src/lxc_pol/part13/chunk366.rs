//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 366/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk366<F: Float>(t28: F, t382: F, t14: F, t400: F, t401: F) -> (F, F, F, F, F) {
    let t1464 = t382 * t28;
    let t1465 = 1.0 / t1464;
    let t1466 = t14 * t1465;
    let t1467 = t400 * t400;
    let t1468 = t1467 * t401;
    let t1469 = t1466 * t1468;
    let t1470 = 2.0 * t1469;
    (t1465, t1466, t1467, t1468, t1470)
}
