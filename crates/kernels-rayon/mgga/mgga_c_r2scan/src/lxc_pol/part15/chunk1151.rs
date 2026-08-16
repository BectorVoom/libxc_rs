//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1151/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1151(t10776: f64, t3308: f64, t8002: f64, t10772: f64, t7945: f64, t37883: f64, t37891: f64, t37893: f64, t37903: f64, t37905: f64, t39786: f64, t39789: f64, t39793: f64, t39795: f64, t39801: f64) -> f64 {
    let t39804 = t10776 * t3308 * t8002;
    let t39807 = t10772 * t3308 * t7945;
    let t39809 = t39786 - 0.16463622957338778997e-1_f64 * t37883 - 0.2600466522016280569e1_f64 * t39789 + t39793 - 0.26198215989259945075e-1_f64 * t39795 - 0.85366933852867742945e0_f64 * t37891 + 0.12805040077930161442e0_f64 * t37893 - 0.31147743054556651236e-1_f64 * t37903 - 0.23804984598836975486e-2_f64 * t37905 + 0.21831846657716620896e-2_f64 * t39801 + 0.86682217400542685632e-1_f64 * t39804 + 0.13002332610081402845e0_f64 * t39807;
    t39809
}
