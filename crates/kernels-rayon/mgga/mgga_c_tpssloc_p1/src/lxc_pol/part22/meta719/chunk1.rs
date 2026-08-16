//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2328/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2328(t13151: f64, t13156: f64, t13160: f64, t1504: f64, t16662: f64, t16736: f64, t16749: f64, t16949: f64, t20756: f64, t20800: f64, t20843: f64, t20846: f64, t20849: f64, t228: f64, t4119: f64, t4225: f64, t4226: f64, t5544: f64, t6589: f64, t67282: f64, t776: f64, t822: f64, t824: f64, t845: f64) -> f64 {
    let t67566 = -360.0_f64 * t20756 * t4225 * t6589 * t776 - 12.0_f64 * t20800 * t4225 * t776 * t845 + 180.0_f64 * t13156 * t16949 * t4225 - 36.0_f64 * t13160 * t4225 * t5544 - 36.0_f64 * t16662 * t4225 * t4226 + 180.0_f64 * t16736 * t4119 * t4225 + 3.0_f64 * t228 * t67282 * t824 - 36.0_f64 * t13151 * t20846 + 9.0_f64 * t1504 * t16749 + 60.0_f64 * t20843 * t822 + 3.0_f64 * t20849 * t822;
    t67566
}
