//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 866/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk866(t551: f64, t552: f64, t9182: f64, t2133: f64, t2139: f64, t2557: f64, t2582: f64, t3101: f64, t3116: f64, t6149: f64, t6152: f64, t6449: f64, t7490: f64, t7496: f64, t9136: f64, t9140: f64, t9144: f64, t9148: f64, t9152: f64, t9156: f64, t9160: f64, t9166: f64, t9170: f64, t9178: f64, t9180: f64) -> f64 {
    let t9184 = t551 * t552 * t9182;
    let t9187 = 0.2600466522016280569e0_f64 * t2139 * t9136 + 0.43341108700271342816e-1_f64 * t2133 * t9140 + 0.13002332610081402845e0_f64 * t2139 * t9144 - 0.86682217400542685632e-1_f64 * t2582 * t9148 - 0.43341108700271342816e-1_f64 * t2582 * t9152 - 0.54878743191129263322e-1_f64 * t2557 * t9156 - 0.27439371595564631661e-1_f64 * t2557 * t9160 + 0.86682217400542685632e-1_f64 * t6149 * t3116 + 0.86682217400542685632e-1_f64 * t2133 * t9166 + 0.86682217400542685632e-1_f64 * t2133 * t9170 + 0.2600466522016280569e0_f64 * t6152 * t3101 + 0.11708928647259339622e0_f64 * t7490 - t7496 - 0.23115257973478049502e0_f64 * t9178 - 0.12805040077930161442e0_f64 * t9180 - 0.5200933044032561138e0_f64 * t6449 * t9184;
    t9187
}
