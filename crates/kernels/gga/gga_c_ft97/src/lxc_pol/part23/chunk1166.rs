//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1166/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1166<F: Float>(t7038: F, t8232: F, t1495: F, t799: F, t24898: F, t56110: F, t29090: F, t8392: F, t10697: F, t1508: F, t2770: F, t1882: F, t29104: F, t7116: F, t29155: F, t46862: F) -> (F, F, F, F, F, F, F, F, F) {
    let t112904 = t8232 * t7038;
    let t112920 = t799 * t1495;
    let t112952 = t56110 * t24898;
    let t112969 = 4.0 / 27.0 * t8392 * t29090;
    let t112975 = t799 * t10697;
    let t112987 = t2770 * t1508;
    let t112992 = 2.0 / 9.0 * t1882 * t29104;
    let t113007 = t8232 * t7116;
    let t113009 = t46862 * t29155;
    (t112904, t112920, t112952, t112969, t112975, t112987, t112992, t113007, t113009)
}
