//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1204/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1204<F: Float>(t25915: F, t376: F, t89: F, t25908: F, t25996: F, t432: F, t446: F, t8411: F, t1564: F, t18: F, t22993: F, t3281: F, t25893: F, t452: F, t46177: F, t5675: F) -> (F, F, F, F, F, F, F) {
    let t101778 = t89 * t376 * t25915;
    let t101779 = 4.0 / 9.0 * t101778;
    let t101781 = t89 * t376 * t25908;
    let t101782 = 4.0 / 9.0 * t101781;
    let t101787 = t446 * t8411 * t25996 * t432;
    let t101791 = t3281 * t1564 * t22993 * t18;
    let t101795 = t25893 * t452 * t5675 * t46177;
    (t101778, t101779, t101781, t101782, t101787, t101791, t101795)
}
