//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1001/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1001(t33942: f64, t33973: f64, t532: f64, t1450: f64, t2014: f64, t1916: f64, t8611: f64, t1518: f64, t8453: f64, t572: f64, t7330: f64, t7741: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t33974 = t33942 + t33973;
    let t33975 = t532 * t33974;
    let t33976 = t33975 * t1450;
    let t33977 = t2014 * t33976;
    let t34003 = 6.0_f64 * t1916 * t8611;
    let t34004 = t1518 * t8453;
    let t34006 = 6.0_f64 * t572 * t34004;
    let t34007 = t7330 * t7741;
    (t33974, t33975, t33976, t33977, t34003, t34004, t34006, t34007)
}
