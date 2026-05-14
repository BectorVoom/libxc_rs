//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 760/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk760<F: Float>(t7276: F, t7288: F, t552: F, t551: F, t2604: F, t5148: F, t5147: F, t506: F, t7088: F, t529: F, t2169: F, t2173: F, t2236: F, t2254: F, t2662: F, t2667: F, t2670: F, t2721: F, t527: F, t549: F, t562: F, t566: F, t568: F, t6487: F, t6490: F, t7235: F, t7237: F, t7240: F, t7245: F, t7250: F, t7259: F, t7263: F, t940: F, t944: F) -> (F, F, F, F) {
    let t7290 = t7276 / 2.0 + t7288 / 2.0;
    let t7291 = t552 * t7290;
    let t7292 = t551 * t7291;
    let t7297 = t5148 * t2604;
    let t7298 = t5147 * t7297;
    let t7300 = t506 * t7088;
    let t7301 = t529 * t7300;
    let t7308 = -0.12713391885412927226e1 * t7235 - 0.42683466926433871473e0 * t7237 - 0.13002332610081402845e0 * t566 * t7240 - 0.86682217400542685632e-1 * t7245 * t562 - 0.43341108700271342816e-1 * t2667 * t2254 - 0.2600466522016280569e0 * t7250 * t568 - 0.13002332610081402845e0 * t2670 * t2173 - 0.2600466522016280569e0 * t2169 * t2662 - 0.63479958930231934629e-2 * t7259 - 0.19043987679069580389e-1 * t7263 - 0.43341108700271342816e-1 * t549 * t7292 - 0.13002332610081402845e0 * t6490 * t944 + 0.81312004494856525156e-4 * t7298 - 0.54878743191129263322e-1 * t527 * t7301 - 0.43341108700271342816e-1 * t6487 * t940 - 0.86682217400542685632e-1 * t2236 * t2721;
    (t7290, t7291, t7301, t7308)
}
