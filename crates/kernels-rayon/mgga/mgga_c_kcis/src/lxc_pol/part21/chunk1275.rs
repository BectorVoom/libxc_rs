//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1275/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1275(t46041: f64, t7743: f64, t95285: f64, t95288: f64, t95291: f64, t95481: f64, t95483: f64, t95485: f64, t95487: f64, t95489: f64, t95491: f64, t95492: f64, t95495: f64, t95498: f64, t95500: f64, t95502: f64, t95503: f64, t95506: f64, t95508: f64) -> (f64, f64) {
    let t95510 = 4.0_f64 * t46041 * t7743;
    let t95511 = -t95285 - t95288 - t95291 - t95481 + t95483 + t95485 + t95487 + t95489 - t95491 - t95492 - t95495 + t95498 - t95500 - t95502 - t95503 - t95506 + t95508 + t95510;
    (t95510, t95511)
}
