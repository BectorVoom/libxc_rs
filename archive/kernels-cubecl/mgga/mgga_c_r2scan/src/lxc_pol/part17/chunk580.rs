//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 580/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk580<F: Float>(t3216: F, t552: F, t551: F, t3016: F, t2184: F, t2196: F, t2223: F, t2667: F, t2670: F, t279: F, t2839: F, t2843: F, t3179: F, t3183: F, t3187: F, t3192: F, t3198: F, t527: F, t549: F, t566: F, t940: F, t944: F) -> (F, F, F, F, F) {
    let t3217 = t552 * t3216;
    let t3218 = t551 * t3217;
    let t3223 = t552 * t3016;
    let t3224 = t551 * t3223;
    let t3227 = F::cast_from(0.43341108700271342816e-1_f64) * t3179 * t279 + F::cast_from(0.5200933044032561138e0_f64) * t2196 * t3183 + F::cast_from(0.16463622957338778997e0_f64) * t2223 * t3187 + F::cast_from(0.86682217400542685632e-1_f64) * t2184 * t3192 + F::cast_from(0.11643651550782197811e-1_f64) * t2839 + F::cast_from(0.10975748638225852664e-1_f64) * t2843 - F::cast_from(0.54878743191129263322e-1_f64) * t527 * t3198 - F::cast_from(0.86682217400542685632e-1_f64) * t2667 * t940 - F::cast_from(0.43341108700271342816e-1_f64) * t549 * t3218 - F::cast_from(0.2600466522016280569e0_f64) * t2670 * t944 - F::cast_from(0.13002332610081402845e0_f64) * t566 * t3224;
    (t3217, t3218, t3223, t3224, t3227)
}
