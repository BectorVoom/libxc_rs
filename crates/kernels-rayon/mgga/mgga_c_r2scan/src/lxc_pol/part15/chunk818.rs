//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 818/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk818(t551: f64, t7291: f64, t2604: f64, t5148: f64, t5147: f64, t506: f64, t7088: f64, t529: f64, t2169: f64, t2173: f64, t2236: f64, t2254: f64, t2662: f64, t2667: f64, t2670: f64, t2721: f64, t527: f64, t549: f64, t562: f64, t566: f64, t568: f64, t6487: f64, t6490: f64, t7235: f64, t7237: f64, t7240: f64, t7245: f64, t7250: f64, t7259: f64, t7263: f64, t940: f64, t944: f64) -> (f64, f64) {
    let t7292 = t551 * t7291;
    let t7297 = t5148 * t2604;
    let t7298 = t5147 * t7297;
    let t7300 = t506 * t7088;
    let t7301 = t529 * t7300;
    let t7308 = -0.12713391885412927226e1_f64 * t7235 - 0.42683466926433871473e0_f64 * t7237 - 0.13002332610081402845e0_f64 * t566 * t7240 - 0.86682217400542685632e-1_f64 * t7245 * t562 - 0.43341108700271342816e-1_f64 * t2667 * t2254 - 0.2600466522016280569e0_f64 * t7250 * t568 - 0.13002332610081402845e0_f64 * t2670 * t2173 - 0.2600466522016280569e0_f64 * t2169 * t2662 - 0.63479958930231934629e-2_f64 * t7259 - 0.19043987679069580389e-1_f64 * t7263 - 0.43341108700271342816e-1_f64 * t549 * t7292 - 0.13002332610081402845e0_f64 * t6490 * t944 + 0.81312004494856525156e-4_f64 * t7298 - 0.54878743191129263322e-1_f64 * t527 * t7301 - 0.43341108700271342816e-1_f64 * t6487 * t940 - 0.86682217400542685632e-1_f64 * t2236 * t2721;
    (t7301, t7308)
}
