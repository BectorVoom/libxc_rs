//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 754/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk754(t33339: f64, t33458: f64, t33474: f64, t33344: f64, t33349: f64, t33455: f64, t33463: f64, t33467: f64, t33471: f64, t33479: f64, t33483: f64, t33487: f64) -> (f64, f64, f64, f64) {
    let t33518 = t33339 / 18.0_f64;
    let t33522 = 2.0_f64 / 9.0_f64 * t33458;
    let t33526 = t33474 / 9.0_f64;
    let t33530 = t33518 + t33344 / 18.0_f64 + t33349 / 3.0_f64 - t33455 / 6.0_f64 - t33522 - 2.0_f64 / 9.0_f64 * t33463 - 2.0_f64 * t33467 + 4.0_f64 / 3.0_f64 * t33471 + t33526 + t33479 / 9.0_f64 + 2.0_f64 / 3.0_f64 * t33483 - t33487 / 3.0_f64;
    (t33518, t33522, t33526, t33530)
}
