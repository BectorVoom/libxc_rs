//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1214/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1214(t14665: f64, t8081: f64, t19843: f64, t26871: f64, t1203: f64, t29081: f64, t3330: f64, t1820: f64, t28071: f64, t10491: f64, t29042: f64, t6735: f64, t7766: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t99837 = 2.0_f64 * t14665 * t8081;
    let t99839 = 2.0_f64 * t26871 * t19843;
    let t99842 = 2.0_f64 * t3330 * t29081 * t1203;
    let t99845 = 4.0_f64 * t3330 * t28071 * t1820;
    let t99847 = 2.0_f64 * t10491 * t29042;
    let t99850 = 2.0_f64 * t3330 * t7766 * t6735;
    (t99837, t99839, t99842, t99845, t99847, t99850)
}
