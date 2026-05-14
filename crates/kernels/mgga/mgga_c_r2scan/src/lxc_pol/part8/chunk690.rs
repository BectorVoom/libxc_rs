//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 690/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk690<F: Float>(t3216: F, t552: F, t551: F, t3016: F, t2184: F, t2196: F, t2223: F, t2667: F, t2670: F, t279: F, t2839: F, t2843: F, t3179: F, t3183: F, t3187: F, t3192: F, t3198: F, t527: F, t549: F, t566: F, t940: F, t944: F) -> (F, F, F) {
    let t3217 = t552 * t3216;
    let t3218 = t551 * t3217;
    let t3223 = t552 * t3016;
    let t3224 = t551 * t3223;
    let t3227 = 0.43341108700271342816e-1 * t3179 * t279 + 0.5200933044032561138e0 * t2196 * t3183 + 0.16463622957338778997e0 * t2223 * t3187 + 0.86682217400542685632e-1 * t2184 * t3192 + 0.11643651550782197811e-1 * t2839 + 0.10975748638225852664e-1 * t2843 - 0.54878743191129263322e-1 * t527 * t3198 - 0.86682217400542685632e-1 * t2667 * t940 - 0.43341108700271342816e-1 * t549 * t3218 - 0.2600466522016280569e0 * t2670 * t944 - 0.13002332610081402845e0 * t566 * t3224;
    (t3218, t3224, t3227)
}
