//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1089/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1089(t358: f64, t7274: f64, t1882: f64, t34729: f64, t34696: f64, t376: f64, t89: f64, t102524: f64, t103472: f64, t103510: f64, t110: f64, t11863: f64, t137713: f64, t138119: f64, t138126: f64, t138143: f64, t144893: f64, t145585: f64, t1825: f64, t1901: f64, t1909: f64, t25924: f64, t25929: f64, t26305: f64, t26318: f64, t26410: f64, t26423: f64, t26441: f64, t3113: f64, t3204: f64, t34482: f64, t34681: f64, t34768: f64, t38711: f64, t39107: f64, t446: f64, t452: f64, t47548: f64, t47666: f64, t499: f64, t5710: f64, t91771: f64, t925: f64) -> f64 {
    let t146766 = t7274 * t358;
    let t146775 = t1882 * t34729;
    let t146803 = t89 * t376 * t34696;
    let t146806 = -4.0_f64 / 27.0_f64 * t47666 * t103510 * t26441 - 2.0_f64 / 9.0_f64 * t1901 * t91771 * t26318 - 4.0_f64 / 9.0_f64 * t1901 * t102524 * t25924 + 4.0_f64 / 27.0_f64 * t1901 * t103472 * t25929 - 2.0_f64 / 9.0_f64 * t1901 * t38711 * t34681 - 4.0_f64 / 9.0_f64 * t1901 * t11863 * t144893 - 2.0_f64 / 9.0_f64 * t1901 * t91771 * t26305 + 2.0_f64 / 9.0_f64 * t1901 * t39107 * t146766 * t3204 + 2.0_f64 / 3.0_f64 * t1901 * t47548 * t146766 * t3113 + t146775 / 9.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t452 * t5710 * t26410 + 2.0_f64 / 3.0_f64 * t446 * t452 * t5710 * t26423 + t138119 + t1901 * t1909 * t137713 * t925 / 9.0_f64 - 4.0_f64 / 9.0_f64 * t138126 + t446 * t452 * t1825 * t34768 / 3.0_f64 - t446 * t452 * t499 * t34482 / 3.0_f64 - t446 * t452 * t110 * t145585 / 3.0_f64 - t146803 / 9.0_f64 - t138143 / 27.0_f64;
    t146806
}
