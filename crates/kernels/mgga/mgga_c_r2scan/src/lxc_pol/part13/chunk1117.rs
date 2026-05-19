//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1117/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1117<F: Float>(t39410: F, t10772: F, t3308: F, t7978: F, t8006: F, t39385: F, t39387: F, t39390: F, t39393: F, t39396: F, t39397: F, t39401: F, t39404: F, t39406: F) -> F {
    let t39411 = F::cast_from(0.47609969197673950972e-2_f64) * t39410;
    let t39413 = t10772 * t3308 * t7978;
    let t39416 = t10772 * t3308 * t8006;
    let t39418 = -F::cast_from(0.43341108700271342816e-1_f64) * t39385 - F::cast_from(0.86682217400542685632e-1_f64) * t39387 + F::cast_from(0.86682217400542685632e-1_f64) * t39390 + F::cast_from(0.2600466522016280569e0_f64) * t39393 + t39396 - F::cast_from(0.27439371595564631661e-1_f64) * t39397 - t39401 - t39404 - F::cast_from(0.43341108700271342816e-1_f64) * t39406 + t39411 + F::cast_from(0.2600466522016280569e0_f64) * t39413 + F::cast_from(0.13002332610081402845e0_f64) * t39416;
    t39418
}
