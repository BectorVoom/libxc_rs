//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1221/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1221(t1077: f64, t1083: f64, t1089: f64, t13286: f64, t13287: f64, t1459: f64, t15386: f64, t17171: f64, t17177: f64, t17198: f64, t1772: f64, t1795: f64, t1849: f64, t22325: f64, t22327: f64, t22329: f64, t22333: f64, t22337: f64, t3169: f64, t3176: f64, t368: f64, t398: f64, t418: f64, t4838: f64, t513: f64, t839: f64) -> f64 {
    let t22339 = -0.13719685797782315831e-1_f64 * t13286 * t13287 * t1849 * t3169 + 0.20579528696673473746e-1_f64 * t13286 * t15386 * t1849 * t3176 - 0.85748036236139473944e-3_f64 * t418 * t398 * t1083 * t1795 * t1077 + 0.25724410870841842184e-2_f64 * t418 * t398 * t1459 * t513 * t4838 + 0.68598428988911579156e-2_f64 * t17171 - 0.34299214494455789578e-2_f64 * t17177 - 0.34299214494455789578e-2_f64 * t17198 - 0.17149607247227894789e-2_f64 * t418 * t1089 * t368 * t1772 * t839 - 0.21437009059034868486e-3_f64 * t22325 + 0.34299214494455789578e-2_f64 * t22327 - 0.17149607247227894789e-2_f64 * t22329 - 0.17149607247227894789e-2_f64 * t22333 - 0.34299214494455789578e-2_f64 * t22337;
    t22339
}
