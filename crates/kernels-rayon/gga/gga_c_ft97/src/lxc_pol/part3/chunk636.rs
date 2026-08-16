//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 636/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk636(t299: f64, t332: f64, t5473: f64, t113: f64, t1273: f64, t1274: f64, t992: f64, t1259: f64, t1275: f64, t333: f64, t4322: f64, t4635: f64, t5: f64, t5430: f64, t889: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t300 = 10000000.0_f64 <= t299;
    let t5474 = t5473 * t332;
    let t5475 = t5474 * t113;
    let t5478 = t1273 * t1273;
    let t5479 = t5478 * t332;
    let t5480 = t5479 * t113;
    let t5483 = t1274 * t992;
    let t5490 = piecewise3(t300, 0.0_f64, t5 * t5430 * t113 / 4.0_f64 + t4322 * t1275 / 2.0_f64 + t5 * t1259 * t992 / 2.0_f64 + t889 * t5475 / 4.0_f64 + t889 * t5480 / 4.0_f64 + t889 * t5483 / 2.0_f64 + t5 * t333 * t4635 / 4.0_f64);
    (t5474, t5475, t5478, t5479, t5480, t5483, t5490)
}
