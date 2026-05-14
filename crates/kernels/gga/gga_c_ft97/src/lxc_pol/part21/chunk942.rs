//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 942/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk942<F: Float>(t29668: F, t29725: F, t103: F, t942: F, t984: F, t5507: F, t28: F, t108: F, t4495: F, t19969: F, t2: F, t4: F, t26: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t29726 = t29668 + t29725;
    let t29727 = t29726 * t103;
    let t29729 = t984 * t942;
    let t29730 = t5507 * t29729;
    let t29731 = t28 * t29730;
    let t29734 = t108 * t4495;
    let t29735 = t5507 * t29734;
    let t29736 = t28 * t29735;
    let t29739 = t19969 * t2;
    let t29740 = t29739 * t4;
    let t29741 = t29740 * t26;
    (t29726, t29727, t29729, t29730, t29731, t29734, t29735, t29736, t29740, t29741)
}
