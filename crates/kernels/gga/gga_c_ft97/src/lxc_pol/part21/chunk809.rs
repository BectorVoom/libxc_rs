//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 809/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk809<F: Float>(t376: F, t5844: F, t1349: F, t5848: F, t1359: F, t614: F, t1361: F, t1637: F, t5766: F, t5769: F, t5: F, t5981: F, t108: F, t3103: F, t5507: F, t28: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t24087 = t376 * t5844;
    let t24088 = t1349 * t24087;
    let t24094 = t376 * t5848;
    let t24095 = t1349 * t24094;
    let t24102 = t1359 * t614;
    let t24116 = t1637 * t1361;
    let t24118 = 2.0 / 27.0 * t1349 * t24116;
    let t24119 = t5766 * t5769;
    let t24157 = t5 * t5981;
    let t25523 = t108 * t3103;
    let t25524 = t5507 * t25523;
    let t25525 = t28 * t25524;
    (t24087, t24088, t24094, t24095, t24102, t24116, t24118, t24119, t24157, t25523, t25524, t25525)
}
