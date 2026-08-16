//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 1043/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk1043(t128521: f64, t2039: f64, t126035: f64, t126036: f64, t126116: f64, t126118: f64, t126120: f64, t128928: f64, t128930: f64, t128932: f64, t128934: f64, t128936: f64, t28951: f64, t6517: f64, t8446: f64, t96686: f64) -> f64 {
    let t128942 = 2.0_f64 * t128521 * t2039;
    let t128943 = 2.0_f64 * t2039 * t96686 + 2.0_f64 * t28951 * t6517 + t126035 + t126036 + t126116 + t126118 + t126120 + t128928 + t128930 + t128932 + t128934 + t128936 + t128942 + t8446;
    t128943
}
