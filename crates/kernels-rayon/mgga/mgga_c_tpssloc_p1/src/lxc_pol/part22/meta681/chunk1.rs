//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2246/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2246(t5889: f64, t698: f64, t973: f64, t10422: f64, t17676: f64, t3070: f64, t17171: f64, t2970: f64, t17167: f64, t10231: f64, t17157: f64, t17161: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t62565 = t973 * t698 * t5889;
    let t62602 = t3070 * t10422 * t17676;
    let t62631 = t973 * t2970 * t17171;
    let t62640 = t973 * t2970 * t17167;
    let t62657 = t973 * t10231 * t17157;
    let t62660 = t973 * t10231 * t17161;
    (t62565, t62602, t62631, t62640, t62657, t62660)
}
