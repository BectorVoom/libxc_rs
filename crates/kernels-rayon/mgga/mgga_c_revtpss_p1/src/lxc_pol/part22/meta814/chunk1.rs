//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2921/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2921(t10175: f64, t9686: f64, t1420: f64, t4075: f64, t786: f64, t2439: f64, t3895: f64, t4132: f64, t1359: f64, t39501: f64, t10115: f64, t555: f64) -> (f64, f64, f64, f64, f64) {
    let t47527 = t10175 * t9686;
    let t47530 = t786 * t1420 * t4075;
    let t47534 = t2439 * t3895 * t4132;
    let t47561 = 0.56911289235245161963e-1_f64 * t39501 * t1359;
    let t47567 = t10115 * t555;
    (t47527, t47530, t47534, t47561, t47567)
}
