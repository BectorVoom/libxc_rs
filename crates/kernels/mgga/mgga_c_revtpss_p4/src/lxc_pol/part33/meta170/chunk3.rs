//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 837/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk837<F: Float>(t1544: F, t262: F, t1583: F, t892: F, t2404: F, t2411: F, t1940: F, t198: F, t207: F, t2403: F, t2621: F, t2628: F, t2632: F, t4316: F, t4343: F, t4394: F, t4396: F, t4397: F, t4400: F, t4405: F, t4406: F, t4537: F, t4541: F, t765: F, t775: F, t890: F) -> (F, F, F, F) {
    let t4542 = t262 * t1544;
    let t4546 = t1583 * t892;
    let t4553 = t2404 * t1544;
    let t4556 = t1583 * t2411;
    let t4559 = t198 * t207 * t4537 * t892 - t1940 * t4556 * t890 + F::cast_from(3.0_f64) * t198 * t4343 * t765 + F::cast_from(3.0_f64) * t2403 * t4546 * t775 + F::cast_from(6.0_f64) * t4541 * t4542 * t775 + F::cast_from(3.0_f64) * t2403 * t4553 + t2621 + t2628 + t2632 + t4316 + t4394 + t4396 + t4397 - t4400 + t4405 - t4406;
    (t4542, t4546, t4556, t4559)
}
