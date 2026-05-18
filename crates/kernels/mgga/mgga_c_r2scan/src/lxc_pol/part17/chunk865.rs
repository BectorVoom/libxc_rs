//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 865/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk865<F: Float>(t551: F, t552: F, t9182: F, t2133: F, t2139: F, t2557: F, t2582: F, t3101: F, t3116: F, t6149: F, t6152: F, t6449: F, t7490: F, t7496: F, t9136: F, t9140: F, t9144: F, t9148: F, t9152: F, t9156: F, t9160: F, t9166: F, t9170: F, t9178: F, t9180: F) -> F {
    let t9184 = t551 * t552 * t9182;
    let t9187 = F::new(0.2600466522016280569e0) * t2139 * t9136 + F::new(0.43341108700271342816e-1) * t2133 * t9140 + F::new(0.13002332610081402845e0) * t2139 * t9144 - F::new(0.86682217400542685632e-1) * t2582 * t9148 - F::new(0.43341108700271342816e-1) * t2582 * t9152 - F::new(0.54878743191129263322e-1) * t2557 * t9156 - F::new(0.27439371595564631661e-1) * t2557 * t9160 + F::new(0.86682217400542685632e-1) * t6149 * t3116 + F::new(0.86682217400542685632e-1) * t2133 * t9166 + F::new(0.86682217400542685632e-1) * t2133 * t9170 + F::new(0.2600466522016280569e0) * t6152 * t3101 + F::new(0.11708928647259339622e0) * t7490 - t7496 - F::new(0.23115257973478049502e0) * t9178 - F::new(0.12805040077930161442e0) * t9180 - F::new(0.5200933044032561138e0) * t6449 * t9184;
    t9187
}
