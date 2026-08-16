//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1271/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1271(t1640: f64, t1881: f64, t2233: f64, t28316: f64, t28877: f64, t29665: f64, t6888: f64, t6895: f64, t7886: f64, t7889: f64, t7892: f64, t8015: f64, t91885: f64, t91895: f64, t91901: f64, t92157: f64, t92379: f64, t97561: f64) -> f64 {
    let t101807 = -t91885 - t29665 * t7886 / 8.0_f64 + t97561 + t1881 * t28877 / 8.0_f64 + t91895 - t6888 * t8015 / 8.0_f64 - t6888 * t7892 / 8.0_f64 - t91901 + t92379 + t1881 * t28316 / 8.0_f64 - t29665 * t7889 / 8.0_f64 + t92157 - t2233 * t6895 * t1640 / 16.0_f64;
    t101807
}
