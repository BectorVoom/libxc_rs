//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1275/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1275<F: Float>(t46041: F, t7743: F, t95285: F, t95288: F, t95291: F, t95481: F, t95483: F, t95485: F, t95487: F, t95489: F, t95491: F, t95492: F, t95495: F, t95498: F, t95500: F, t95502: F, t95503: F, t95506: F, t95508: F) -> (F, F) {
    let t95510 = F::cast_from(4.0_f64) * t46041 * t7743;
    let t95511 = -t95285 - t95288 - t95291 - t95481 + t95483 + t95485 + t95487 + t95489 - t95491 - t95492 - t95495 + t95498 - t95500 - t95502 - t95503 - t95506 + t95508 + t95510;
    (t95510, t95511)
}
