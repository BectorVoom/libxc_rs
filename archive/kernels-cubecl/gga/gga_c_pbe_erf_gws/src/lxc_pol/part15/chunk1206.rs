//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1206/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1206<F: Float>(t13861: F, t51666: F, t1193: F, t353: F, t6161: F, t859: F, t13918: F, t2249: F, t13952: F, t2210: F, t14122: F, t14125: F, t2113: F, t833: F, t850: F) -> (F, F, F, F, F, F) {
    let t51667 = t51666 * t13861;
    let t51675 = t859 * t353 * t1193 * t6161;
    let t51678 = t2249 * t13918;
    let t51682 = t13952 * t2210;
    let t51683 = t51682 * t14122;
    let t51688 = t850 * t2113 * t14125 * t833;
    (t51667, t51675, t51678, t51682, t51683, t51688)
}
