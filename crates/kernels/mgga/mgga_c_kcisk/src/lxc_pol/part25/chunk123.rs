//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 123/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk123<F: Float>(t45: F, t608: F, t621: F, t625: F, t634: F, t67: F, t227: F, t8: F) -> (F, F, F, F) {
    let t638 = -0.62182e-1 * t608 * t621 + 0.19751789702565206229e-1 * t45 * t625 * t634;
    let t639 = t67 * t638;
    let t640 = t8 * t227;
    let t641 = pow_1_3(t640);
    (t638, t639, t640, t641)
}
