//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 619/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk619(t299: f64, t332: f64, t4375: f64, t113: f64, t1273: f64, t909: f64, t1274: f64, t505: f64, t910: f64, t992: f64, t18: f64, t1577: f64, t1259: f64, t1275: f64, t2904: f64, t4318: f64, t4322: f64, t5: f64, t886: f64, t889: f64, t911: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t300 = 10000000.0_f64 <= t299;
    let t4376 = t4375 * t332;
    let t4377 = t4376 * t113;
    let t4380 = t1273 * t909;
    let t4381 = t332 * t113;
    let t4382 = t4380 * t4381;
    let t4385 = t1274 * t505;
    let t4391 = t910 * t992;
    let t4394 = t332 * t18;
    let t4395 = t4394 * t1577;
    let t4399 = piecewise3(t300, 0.0_f64, t5 * t4318 * t113 / 4.0_f64 + t4322 * t911 / 4.0_f64 + t5 * t1259 * t505 / 4.0_f64 + t2904 * t1275 / 4.0_f64 + t889 * t4377 / 4.0_f64 + t889 * t4382 / 4.0_f64 + t889 * t4385 / 4.0_f64 + t5 * t886 * t992 / 4.0_f64 + t889 * t4391 / 4.0_f64 - t889 * t4395 / 2.0_f64);
    (t4376, t4377, t4380, t4381, t4382, t4385, t4391, t4394, t4395, t4399)
}
