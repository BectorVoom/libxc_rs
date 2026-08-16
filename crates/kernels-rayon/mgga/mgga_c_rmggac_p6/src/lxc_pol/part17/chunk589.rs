//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 589/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk589(t2147: f64, t8368: f64, t1624: f64, t665: f64, t1550: f64, t1627: f64, t903: f64, t352: f64, t551: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8369 = t8368 * t2147;
    let t8371 = t665 * t1624;
    let t8372 = t1550 * t8371;
    let t8374 = t665 * t1627;
    let t8375 = t903 * t8374;
    let t8377 = t551 * t352;
    (t8369, t8371, t8372, t8374, t8375, t8377)
}
