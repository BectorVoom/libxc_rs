//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1267/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1267(t196: f64, t197: f64, t28686: f64, t2035: f64, t34270: f64, t7313: f64, t28021: f64, t8698: f64, t27833: f64, t8715: f64, t32735: f64, t7898: f64) -> (f64, f64, f64, f64, f64) {
    let t128958 = t28686 * t196 * t197;
    let t128959 = t128958 * t2035;
    let t128960 = t34270 * t7313;
    let t128964 = t8698 * t28021;
    let t128965 = t27833 * t8715;
    let t128966 = t7898 * t32735;
    (t128959, t128960, t128964, t128965, t128966)
}
