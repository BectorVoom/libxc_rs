//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 477/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk477(t184: f64, t4888: f64, t21: f64, t1078: f64, t1079: f64, t920: f64, t2321: f64, t992: f64, t1131: f64, t231: f64, t1137: f64, t1526: f64, t2319: f64, t2320: f64, t342: f64, t343: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4889 = t4888 * t184;
    let t4890 = t4889 * t21;
    let t4893 = t1078 * t1078;
    let t4894 = t4893 * t184;
    let t4895 = t4894 * t21;
    let t4898 = t1079 * t920;
    let t4906 = t2321 * t992;
    let t4910 = t231 * t1131;
    let t4914 = t1137 - t2319 - t1526 * t2320 * t4906 / 12.0_f64 - t342 * t343 * t4910 / 4.0_f64;
    (t4889, t4890, t4893, t4894, t4895, t4898, t4906, t4910, t4914)
}
