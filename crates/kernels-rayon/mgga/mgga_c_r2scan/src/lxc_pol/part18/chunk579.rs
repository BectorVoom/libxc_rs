//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 579/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk579(t3216: f64, t552: f64, t551: f64, t3016: f64, t2184: f64, t2196: f64, t2223: f64, t2667: f64, t2670: f64, t279: f64, t2839: f64, t2843: f64, t3179: f64, t3183: f64, t3187: f64, t3192: f64, t3198: f64, t527: f64, t549: f64, t566: f64, t940: f64, t944: f64) -> (f64, f64, f64, f64, f64) {
    let t3217 = t552 * t3216;
    let t3218 = t551 * t3217;
    let t3223 = t552 * t3016;
    let t3224 = t551 * t3223;
    let t3227 = 0.43341108700271342816e-1_f64 * t3179 * t279 + 0.5200933044032561138e0_f64 * t2196 * t3183 + 0.16463622957338778997e0_f64 * t2223 * t3187 + 0.86682217400542685632e-1_f64 * t2184 * t3192 + 0.11643651550782197811e-1_f64 * t2839 + 0.10975748638225852664e-1_f64 * t2843 - 0.54878743191129263322e-1_f64 * t527 * t3198 - 0.86682217400542685632e-1_f64 * t2667 * t940 - 0.43341108700271342816e-1_f64 * t549 * t3218 - 0.2600466522016280569e0_f64 * t2670 * t944 - 0.13002332610081402845e0_f64 * t566 * t3224;
    (t3217, t3218, t3223, t3224, t3227)
}
