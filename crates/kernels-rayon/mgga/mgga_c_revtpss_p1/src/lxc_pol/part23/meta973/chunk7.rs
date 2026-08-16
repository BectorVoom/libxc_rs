//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3305/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3305(t1343: f64, t13600: f64, t1450: f64, t1868: f64, t198: f64, t22466: f64, t22486: f64, t39419: f64, t39422: f64, t4139: f64, t46297: f64, t46963: f64, t46970: f64, t47753: f64, t47760: f64, t48157: f64, t48159: f64, t532: f64, t5536: f64, t5591: f64, t6836: f64, t75379: f64, t85390: f64, t85391: f64, t85442: f64, t85466: f64, t85482: f64, t85498: f64, t85887: f64, t85888: f64, t85889: f64, t86291: f64, t86308: f64, t86340: f64, t86691: f64, t86718: f64) -> f64 {
    let t86728 = t47753 + t85390 - t85391 - t47760 + 18.0_f64 * t5536 * t13600 * t6836 + 18.0_f64 * t5536 * t75379 * t1868 + 18.0_f64 * t5536 * t22486 * t5591 - t46297 + 3.0_f64 * t198 * t1343 * t85442 + t198 * t532 * (t85466 + t85482 + t85498 + t86291 + t86308 + t86340 + t86691 + t86718) * t1450 - t39419 - t39422 + t85887 - 9.0_f64 * t4139 * t22466 * t5591 - t85888 - t85889 - t48157 + t48159 - t46963 + t46970;
    t86728
}
