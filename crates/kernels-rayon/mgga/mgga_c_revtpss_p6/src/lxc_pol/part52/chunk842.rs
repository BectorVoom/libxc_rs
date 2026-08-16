//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 842/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk842(t25253: f64, t1945: f64, t2693: f64, t807: f64, t2718: f64, t64: f64, t7036: f64, t820: f64, t843: f64, t839: f64, t241: f64, t159: f64, t2698: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t25254 = 0.15244095330869239812e-3_f64 * t25253;
    let t25255 = t1945 * t2693;
    let t25256 = t807 * t25255;
    let t25257 = 0.11433071498151929859e-3_f64 * t25256;
    let t25260 = t2718 * t64;
    let t25266 = t820 * t7036 * t843;
    let t25267 = t25266 * t839;
    let t25270 = t820 * t7036 * t241;
    let t25273 = t2698 * t159;
    (t25254, t25256, t25257, t25260, t25266, t25267, t25270, t25273)
}
