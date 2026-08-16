//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3313/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3313(t1312: f64, t13426: f64, t1518: f64, t18227: f64, t18245: f64, t21881: f64, t22633: f64, t2322: f64, t27123: f64, t28219: f64, t30138: f64, t4248: f64, t4292: f64, t5523: f64, t5920: f64, t670: f64, t75439: f64, t75931: f64, t75941: f64, t7889: f64, t85308: f64, t85329: f64, t85360: f64) -> f64 {
    let t86889 = 2.0_f64 * t1312 * t75931 + 6.0_f64 * t13426 * t5920 + 6.0_f64 * t1518 * t75439 + 6.0_f64 * t1518 * t85360 + 6.0_f64 * t18227 * t5920 + 6.0_f64 * t18245 * t4292 + 6.0_f64 * t21881 * t4248 + 6.0_f64 * t21881 * t7889 + 2.0_f64 * t22633 * t2322 + 2.0_f64 * t22633 * t5523 + 6.0_f64 * t27123 * t5920 + 6.0_f64 * t28219 * t5920 + 12.0_f64 * t30138 * t4292 + 2.0_f64 * t670 * t75941 + t85308 + 6.0_f64 * t85329;
    t86889
}
