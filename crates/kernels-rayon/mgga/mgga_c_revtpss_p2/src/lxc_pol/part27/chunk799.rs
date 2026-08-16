//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 799/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk799(t555: f64, t9646: f64, t1358: f64, t22: f64, t1444: f64, t4131: f64, t4076: f64, t1425: f64, t225: f64, t4077: f64, t3907: f64, t9285: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9647 = t9646 * t555;
    let t9648 = t1358 * t22;
    let t9650 = 0.19637199382202157274e-3_f64 * t9647 * t9648;
    let t9651 = t1444 * t4131;
    let t9652 = t4076 * t9651;
    let t9655 = t1425 * t1425;
    let t9656 = 1.0_f64 / t9655;
    let t9657 = t225 * t9656;
    let t9658 = t4077 * t1444;
    let t9659 = t9657 * t9658;
    let t9664 = t3907 * t9285;
    (t9648, t9650, t9652, t9655, t9656, t9658, t9659, t9664)
}
