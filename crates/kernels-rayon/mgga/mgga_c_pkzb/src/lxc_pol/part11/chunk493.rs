//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 493/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk493(t2363: f64, t410: f64, t2126: f64, t2370: f64, t914: f64, t937: f64, t2393: f64, t394: f64, t418: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2433 = t2363 * t410;
    let t2435 = t2126 * t2370;
    let t2439 = t914 * t937;
    let t2446 = t2393 * t410;
    let t2447 = t2126 * t394;
    let t2463 = t418 * t418;
    let t2464 = 1.0_f64 / t2463;
    (t2433, t2435, t2439, t2446, t2447, t2463, t2464)
}
