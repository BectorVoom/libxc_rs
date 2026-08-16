//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 692/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk692(t1497: f64, t84: f64, t77: f64, t1470: f64, t603: f64, t1469: f64, t6968: f64, t6971: f64) -> (f64, f64, f64, f64) {
    let t7705 = t84 * t1497;
    let t7706 = t77 * t7705;
    let t7709 = t603 * t1470;
    let t7714 = 5.0_f64 / 6.0_f64 * t6968 * t1469 + t6971;
    (t7705, t7706, t7709, t7714)
}
