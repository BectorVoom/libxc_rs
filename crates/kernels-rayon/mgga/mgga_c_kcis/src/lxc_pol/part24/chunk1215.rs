//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1215/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1215(t19839: f64, t26871: f64, t10491: f64, t29039: f64, t14668: f64, t28009: f64, t20210: f64, t2189: f64, t3330: f64, t28005: f64, t10498: f64, t1203: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t99852 = 4.0_f64 * t26871 * t19839;
    let t99854 = 4.0_f64 * t10491 * t29039;
    let t99856 = 4.0_f64 * t14668 * t28009;
    let t99859 = 2.0_f64 * t3330 * t2189 * t20210;
    let t99861 = 4.0_f64 * t14668 * t28005;
    let t99864 = 12.0_f64 * t10498 * t29039 * t1203;
    (t99852, t99854, t99856, t99859, t99861, t99864)
}
