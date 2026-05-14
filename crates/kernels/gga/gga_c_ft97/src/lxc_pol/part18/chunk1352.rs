//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1352/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1352<F: Float>(t105510: F, t105516: F, t105505: F, t105508: F, t105514: F, t105520: F, t105524: F, t105528: F, t105532: F, t105536: F, t95107: F, t105543: F, t105559: F, t105567: F, t105541: F, t105548: F, t105552: F, t105557: F, t105564: F, t105570: F, t105574: F, t105581: F, t105584: F) -> (F, F) {
    let t106009 = 2.0 / 81.0 * t105510;
    let t106011 = 2.0 / 27.0 * t105516;
    let t106018 = -t105505 / 3.0 - 2.0 * t105508 - t106009 - t105514 / 9.0 + t106011 + 2.0 / 9.0 * t105520 - t105524 / 18.0 + t95107 / 24.0 - 4.0 / 9.0 * t105528 - t105532 / 18.0 - 2.0 / 9.0 * t105536;
    let t106020 = 2.0 / 3.0 * t105543;
    let t106024 = 4.0 / 27.0 * t105559;
    let t106026 = 2.0 / 9.0 * t105567;
    let t106031 = t105541 / 18.0 + t106020 + t105548 / 24.0 - 2.0 / 9.0 * t105552 - t105557 / 6.0 + t106024 + t105564 / 12.0 - t106026 + 8.0 / 27.0 * t105570 + t105574 / 9.0 + 2.0 / 3.0 * t105581 + 4.0 / 9.0 * t105584;
    (t106018, t106031)
}
