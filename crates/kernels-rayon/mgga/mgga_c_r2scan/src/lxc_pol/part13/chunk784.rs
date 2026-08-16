//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 784/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk784(t357: f64, t761: f64, t366: f64, t2281: f64, t2292: f64, t2287: f64, t757: f64, t6007: f64, t2289: f64, t2300: f64, t862: f64, t2304: f64, t268: f64, t270: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6827 = t761 * t357;
    let t6828 = t6827 * t366;
    let t6831 = t2281 * t2292;
    let t6835 = 1.0_f64 / t2287 / t757;
    let t6836 = t6835 * t6007;
    let t6839 = t2289 * t2292;
    let t6842 = t357 * t2300;
    let t6843 = t862 * t6842;
    let t6845 = t268 * t270 * t2304;
    (t6828, t6831, t6836, t6839, t6843, t6845)
}
