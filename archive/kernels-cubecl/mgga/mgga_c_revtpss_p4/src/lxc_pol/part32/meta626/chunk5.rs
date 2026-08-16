//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1995/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1995<F: Float>(t108879: F, t2047: F, t101237: F, t101240: F, t101850: F, t108872: F, t108876: F, t108945: F, t108952: F, t2048: F, t26175: F, t28154: F, t28628: F, t29513: F, t29551: F, t7352: F, t92568: F, t95253: F, t95255: F, t95316: F) -> F {
    let t109911 = t2047 * t108879;
    let t109918 = -F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t108945 * t2048 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t29551 * t7352 + t108952 * t2048 / F::cast_from(3.0_f64) + t29513 * t7352 / F::cast_from(3.0_f64) - t95253 + F::cast_from(88.0_f64) / F::cast_from(27.0_f64) * t95255 + F::cast_from(20.0_f64) / F::cast_from(3.0_f64) * t28154 * t101850 - F::cast_from(70.0_f64) * t95316 * t108872 + F::cast_from(20.0_f64) * t26175 * t108876 - F::cast_from(20.0_f64) * t92568 * t109911 + F::cast_from(20.0_f64) / F::cast_from(3.0_f64) * t101237 * t28628 + F::cast_from(20.0_f64) / F::cast_from(3.0_f64) * t101240 * t28628;
    t109918
}
