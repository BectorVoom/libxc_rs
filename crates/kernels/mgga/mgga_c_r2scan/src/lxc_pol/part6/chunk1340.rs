//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1340/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1340<F: Float>(t25299: F, t6064: F, t20762: F, t113: F, t24165: F, t6085: F, t6086: F, t24118: F, t6093: F, t1593: F, t20380: F, t2133: F, t2139: F, t2184: F, t25261: F, t25263: F, t25275: F, t25283: F, t25288: F, t25297: F, t2531: F, t2573: F, t5108: F, t5109: F, t551: F, t552: F, t6293: F, t6428: F, t6566: F, t7204: F, t7321: F, t7337: F, t921: F, t938: F) -> (F, F, F) {
    let t25300 = t25299 * t6064;
    let t25301 = t20762 * t25300;
    let t25303 = t24165 * t113;
    let t25305 = t6085 * t6086 * t25303;
    let t25307 = t24118 * t113;
    let t25309 = t6093 * t6086 * t25307;
    let t25311 = 0.1536604809351619373e1 * t25261 + 0.9878173774403267398e0 * t6293 * t7337 * t25263 + 0.13002332610081402845e0 * t2133 * t5109 * t7204 * t2573 - 0.7801399566048841707e0 * t5108 * t5109 * t2531 * t1593 - 0.49390868872016336991e0 * t6293 * t7321 * t25275 - 0.39006997830244208535e0 * t5108 * t5109 * t921 * t6428 + 0.39006997830244208535e0 * t2139 * t5109 * t25283 + 0.27439371595564631661e-2 * t20380 - 0.13869154784086829701e1 * t25288 + 0.86682217400542685632e-1 * t2184 * t551 * t552 * t938 * t6566 - 0.13869154784086829701e1 * t25297 + 0.9878173774403267398e-1 * t25301 + 0.17465477326173296717e-1 * t25305 + 0.52396431978519890151e-1 * t25309;
    (t25300, t25307, t25311)
}
