//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1117/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1117(t39410: f64, t10772: f64, t3308: f64, t7978: f64, t8006: f64, t39385: f64, t39387: f64, t39390: f64, t39393: f64, t39396: f64, t39397: f64, t39401: f64, t39404: f64, t39406: f64) -> f64 {
    let t39411 = 0.47609969197673950972e-2_f64 * t39410;
    let t39413 = t10772 * t3308 * t7978;
    let t39416 = t10772 * t3308 * t8006;
    let t39418 = -0.43341108700271342816e-1_f64 * t39385 - 0.86682217400542685632e-1_f64 * t39387 + 0.86682217400542685632e-1_f64 * t39390 + 0.2600466522016280569e0_f64 * t39393 + t39396 - 0.27439371595564631661e-1_f64 * t39397 - t39401 - t39404 - 0.43341108700271342816e-1_f64 * t39406 + t39411 + 0.2600466522016280569e0_f64 * t39413 + 0.13002332610081402845e0_f64 * t39416;
    t39418
}
