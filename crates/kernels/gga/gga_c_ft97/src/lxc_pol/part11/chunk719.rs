//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 719/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk719<F: Float>(t10388: F, t192: F, t852: F, t10478: F, t2: F, t10410: F, t1775: F, t2772: F, t2775: F, t10589: F, t10591: F, t10594: F, t10595: F, t10597: F, t10600: F, t10604: F, t10607: F, t462: F, t92: F) -> (F, F, F, F) {
    let t10611 = t192 * t852 * t10388;
    let t10613 = t10478 * t2;
    let t10614 = t10613 * t10410;
    let t10617 = t1775 * t2772;
    let t10619 = t1775 * t2775;
    let t10621 = t10589 / 3.0 + 2.0 / 9.0 * t10591 - t10594 - 4.0 / 3.0 * t10595 - 2.0 * t462 * t10597 + 2.0 * t462 * t10600 - 2.0 * t462 * t10604 - 2.0 * t462 * t10607 - t92 * t10611 + 2.0 / 3.0 * t462 * t10614 - 2.0 / 3.0 * t10617 - 2.0 / 3.0 * t10619;
    (t10611, t10613, t10614, t10621)
}
