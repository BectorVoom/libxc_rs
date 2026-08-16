//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2594/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2594(t19033: f64, t4993: f64, t19046: f64, t5018: f64, t5023: f64, t6169: f64, t11546: f64, t1174: f64, t1218: f64, t1227: f64, t1230: f64, t1232: f64, t15498: f64, t1737: f64, t1748: f64, t19026: f64, t19087: f64, t22214: f64, t22218: f64, t248: f64, t3490: f64, t4889: f64, t5014: f64, t5030: f64, t6211: f64, t66147: f64, t66150: f64, t71148: f64, t71158: f64) -> f64 {
    let t72302 = t19033 * t4993;
    let t72304 = t19046 * t5018;
    let t72307 = t6169 * t5023;
    let t72333 = -19.0_f64 / 1296.0_f64 * t72302 - t72304 * t1218 / 192.0_f64 + t72307 * t1232 / 288.0_f64 + t15498 * t6211 / 144.0_f64 - t3490 * t22214 / 4608.0_f64 - t1227 * t248 * t1230 * t71148 / 4608.0_f64 - t3490 * t22218 / 768.0_f64 + 19.0_f64 / 576.0_f64 * t66147 * t1737 + 19.0_f64 / 576.0_f64 * t19026 * t5014 - 19.0_f64 / 864.0_f64 * t66150 * t1748 - 19.0_f64 / 864.0_f64 * t19033 * t5030 - 7.0_f64 / 216.0_f64 * t1174 * t11546 * t71158 + t4889 * t19087 / 9.0_f64;
    t72333
}
