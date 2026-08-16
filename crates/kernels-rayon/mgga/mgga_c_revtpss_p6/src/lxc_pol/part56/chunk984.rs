//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 984/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk984(t2014: f64, t33597: f64, t13272: f64, t8435: f64, t1497: f64, t8441: f64, t8621: f64, t1469: f64, t32143: f64, t1493: f64, t84: f64, t1501: f64, t8453: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t33599 = 3.0_f64 * t2014 * t33597;
    let t33609 = t13272 * t8435;
    let t33612 = t8441 * t1497;
    let t33613 = t8621 * t33612;
    let t33617 = t8621 * t32143 * t1469;
    let t33624 = t84 * t1493;
    let t33625 = t8621 * t33624;
    let t33639 = t1501 * t8453;
    (t33599, t33609, t33612, t33613, t33617, t33624, t33625, t33639)
}
