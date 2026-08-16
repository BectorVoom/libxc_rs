//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2858/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2858(t1222: f64, t3693: f64, t697: f64, t12256: f64, t3698: f64, t3362: f64, t414: f64, t3551: f64, t3565: f64, t225: f64, t480: f64, t12884: f64, t828: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t44343 = t1222 * t697 * t3693;
    let t44348 = t3698 * t12256;
    let t44361 = 1.0_f64 / t414 / t3362;
    let t44420 = t3551 * t3565;
    let t44421 = t44420 * t225;
    let t44422 = t44421 * t480;
    let t44425 = t828 * t12884;
    (t44343, t44348, t44361, t44420, t44421, t44422, t44425)
}
