//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 781/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk781<F: Float>(t1013: F, t554: F, t1045: F, t2097: F, t1614: F, t51: F, t432: F, t497: F, t5507: F, t28: F, t108: F, t1755: F, t492: F, t5743: F, t1852: F, t1332: F, t1820: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t16762 = t1013 * t554;
    let t17164 = t2097 * t1045;
    let t17839 = t51 * t1614;
    let t22493 = t497 * t432;
    let t22494 = t5507 * t22493;
    let t22495 = t28 * t22494;
    let t22498 = t108 * t1755;
    let t22499 = t5507 * t22498;
    let t22500 = t28 * t22499;
    let t22503 = t5743 * t492;
    let t22504 = t1852 * t22503;
    let t22506 = t1332 * t1820;
    (t16762, t17164, t17839, t22493, t22494, t22495, t22498, t22499, t22500, t22503, t22504, t22506)
}
