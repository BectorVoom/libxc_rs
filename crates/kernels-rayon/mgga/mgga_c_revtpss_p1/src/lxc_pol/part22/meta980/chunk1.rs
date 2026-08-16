//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3303/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3303(t6016: f64, t860: f64, t231: f64, t2782: f64, t2783: f64, t18657: f64, t686: f64, t72: f64, t874: f64, t1559: f64, t4423: f64, t2797: f64) -> (f64, f64, f64, f64) {
    let t62612 = t860 * t6016;
    let t62615 = t2782 * t2783 * t62612 * t231;
    let t62619 = t874 * t18657 * t72 * t686;
    let t62624 = t1559 * t4423;
    let t62626 = t2782 * t2797 * t62624;
    (t62612, t62615, t62619, t62626)
}
