//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 814/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk814(t240: f64, t7262: f64, t3994: f64, t2661: f64, t2482: f64, t27: f64, t4021: f64, t25273: f64, t533: f64, t816: f64, t540: f64, t7021: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t25986 = t7262 * t240;
    let t25987 = t25986 * t3994;
    let t25988 = t2661 * t25987;
    let t25997 = t2482 * t7262 * t27;
    let t25998 = t25997 * t4021;
    let t26002 = t25273 * t533 * t816;
    let t26003 = 35.0_f64 / 432.0_f64 * t26002;
    let t26004 = t7021 * t540;
    (t25986, t25988, t25997, t25998, t26003, t26004)
}
