//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1559/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1559(t24648: f64, t3172: f64, t3711: f64, t1261: f64, t24228: f64, t247: f64, t44895: f64, t20820: f64, t5265: f64, t20851: f64, t5362: f64, t21101: f64, t5273: f64) -> (f64, f64, f64, f64, f64) {
    let t83539 = t3711 * t3172 * t24648;
    let t83558 = t1261 * t247 * t44895 * t24228;
    let t83580 = t20820 * t5265;
    let t83584 = t20851 * t5362;
    let t83603 = t5273 * t21101;
    (t83539, t83558, t83580, t83584, t83603)
}
