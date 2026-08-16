//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2595/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2595(t18321: f64, t5040: f64, t1174: f64, t1177: f64, t1748: f64, t19002: f64, t19005: f64, t19047: f64, t19051: f64, t4889: f64, t5014: f64, t5030: f64, t52628: f64, t65581: f64, t65598: f64, t65600: f64, t65605: f64, t65607: f64, t65613: f64, t71168: f64, t71177: f64) -> f64 {
    let t72352 = t18321 * t5040;
    let t72357 = t4889 * t19005 / 6.0_f64 - t1174 * t1177 * t71177 / 144.0_f64 - t1174 * t1177 * t71168 / 16.0_f64 - t65581 / 4608.0_f64 + 5.0_f64 / 3456.0_f64 * t65598 + t52628 * t19002 / 72.0_f64 + t19047 * t5014 / 1024.0_f64 - t65607 * t1748 / 1536.0_f64 - t19051 * t5030 / 1536.0_f64 - 11.0_f64 / 324.0_f64 * t72352 + t65600 / 432.0_f64 - t65605 / 2304.0_f64 - t65613 / 1152.0_f64;
    t72357
}
