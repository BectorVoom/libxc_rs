//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 796/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk796<F: Float>(t1632: F, t3071: F, t551: F, t1577: F, t1600: F, t3073: F, t2892: F, t560: F, t552: F, t2133: F, t2139: F, t2557: F, t2582: F, t3101: F, t3116: F, t6149: F, t6152: F, t6449: F, t7490: F, t7496: F, t9136: F, t9140: F, t9144: F, t9148: F, t9152: F, t9156: F, t9160: F, t9166: F, t9170: F) -> (F,) {
    let t9177 = t551 * t1632 * t3071;
    let t9178 = t1577 * t9177;
    let t9180 = t1600 * t3073;
    let t9182 = t2892 * t560;
    let t9184 = t551 * t552 * t9182;
    let t9187 = 0.2600466522016280569e0 * t2139 * t9136 + 0.43341108700271342816e-1 * t2133 * t9140 + 0.13002332610081402845e0 * t2139 * t9144 - 0.86682217400542685632e-1 * t2582 * t9148 - 0.43341108700271342816e-1 * t2582 * t9152 - 0.54878743191129263322e-1 * t2557 * t9156 - 0.27439371595564631661e-1 * t2557 * t9160 + 0.86682217400542685632e-1 * t6149 * t3116 + 0.86682217400542685632e-1 * t2133 * t9166 + 0.86682217400542685632e-1 * t2133 * t9170 + 0.2600466522016280569e0 * t6152 * t3101 + 0.11708928647259339622e0 * t7490 - t7496 - 0.23115257973478049502e0 * t9178 - 0.12805040077930161442e0 * t9180 - 0.5200933044032561138e0 * t6449 * t9184;
    (t9187,)
}
