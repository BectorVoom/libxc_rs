//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1171/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1171<F: Float>(t25883: F, t925: F, t93378: F, t93379: F, t101678: F, t28: F, t89: F, t942: F, t25893: F, t452: F, t4533: F, t473: F, t5675: F, t25899: F, t3266: F, t5674: F, t8411: F) -> (F, F, F, F) {
    let t116679 = t93378 * t93379 * t925 * t25883;
    let t116683 = t89 * t28 * t101678 * t942;
    let t116688 = t25893 * t452 * t5675 * t4533 * t473;
    let t116692 = t5674 * t8411 * t25899 * t3266;
    (t116679, t116683, t116688, t116692)
}
