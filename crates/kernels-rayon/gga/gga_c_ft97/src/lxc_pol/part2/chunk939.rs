//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 939/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk939(t14483: f64, t14560: f64, t332: f64, t113: f64, t10829: f64, t1275: f64, t14391: f64, t14395: f64, t14403: f64, t14409: f64, t14412: f64, t2904: f64, t2963: f64, t4322: f64, t4377: f64, t4382: f64, t4385: f64, t4391: f64, t4395: f64, t889: f64) -> f64 {
    let t14561 = t14483 + t14560;
    let t14562 = t14561 * t332;
    let t14563 = t14562 * t113;
    let t14568 = t889 * t14391 / 4.0_f64 + t889 * t14395 / 4.0_f64 + t2904 * t4377 / 2.0_f64 + t2904 * t4385 / 2.0_f64 + t889 * t14403 / 2.0_f64 + t2904 * t4382 / 2.0_f64 + t889 * t14409 / 2.0_f64 + t889 * t14412 / 4.0_f64 + t2904 * t4391 / 2.0_f64 - t2904 * t4395 + t10829 * t1275 / 4.0_f64 + t889 * t14563 / 4.0_f64 + t4322 * t2963 / 4.0_f64;
    t14568
}
