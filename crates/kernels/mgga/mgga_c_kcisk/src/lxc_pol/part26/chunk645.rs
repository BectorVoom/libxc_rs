//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 645/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk645<F: Float>(t1588: F, t2326: F, t1163: F, t1312: F, t4374: F, t1591: F, t6204: F, t3573: F, t4423: F, t5668: F, t5673: F, t5678: F, t5682: F, t1528: F, t2285: F, t1536: F, t2293: F) -> (F, F, F, F, F, F, F, F) {
    let t6500 = t1588 * t2326;
    let t6501 = t6500 * t1163;
    let t6502 = t1312 * t6501;
    let t6505 = t4374 * t2326;
    let t6506 = t6505 * t1591;
    let t6507 = t6204 * t6506;
    let t6515 = t4423 + 0.57077777777777777777e-2 * t3573 + 0.57077777777777777777e-2 * t5668 - 0.11415555555555555555e-1 * t5673 + 0.34246666666666666666e-1 * t5678 - 0.34246666666666666666e-1 * t5682;
    let t6518 = t2285 * t1528;
    let t6523 = t2293 * t1536;
    (t6501, t6502, t6505, t6506, t6507, t6515, t6518, t6523)
}
