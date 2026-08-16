//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1611/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1611(t44306: f64, t44319: f64, t459: f64, t1256: f64, t12890: f64, t3588: f64, t482: f64, t1222: f64, t3693: f64, t697: f64, t13021: f64, t140: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t44321 = (t44306 + t44319) * t459;
    let t44326 = t12890 * t1256;
    let t44332 = t3588 * t3588;
    let t44333 = t482 * t44332;
    let t44343 = t1222 * t697 * t3693;
    let t44346 = t1222 * t140 * t13021;
    (t44321, t44326, t44332, t44333, t44343, t44346)
}
