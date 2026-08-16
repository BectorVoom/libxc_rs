//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1299/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1299(t117693: f64, t117695: f64, t118335: f64, t118337: f64, t125074: f64, t125966: f64, t125970: f64, t125975: f64, t1404: f64, t1858: f64, t2174: f64, t27908: f64, t3: f64, t32630: f64, t34386: f64, t5381: f64, t580: f64, t8920: f64) -> f64 {
    let t125979 = t125966 * t3 * t580 + t1404 * t34386 + t1858 * t32630 + 2.0_f64 * t2174 * t27908 + t5381 * t8920 + t117693 + t117695 + 2.0_f64 * t118335 + 2.0_f64 * t118337 + t125074 + t125970 + 2.0_f64 * t125975;
    t125979
}
