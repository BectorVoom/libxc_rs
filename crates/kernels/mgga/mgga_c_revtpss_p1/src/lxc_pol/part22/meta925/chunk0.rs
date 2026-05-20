//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3147/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3147<F: Float>(t12640: F, t488: F, t17588: F, t3172: F, t3711: F, t1261: F, t17699: F, t17720: F, t3647: F, t12904: F, t5274: F, t12959: F, t17505: F) -> (F, F, F, F, F, F) {
    let t56707 = t12640 * t488;
    let t56713 = t3711 * t3172 * t17588;
    let t56718 = t1261 * t3172 * t17699;
    let t56720 = t3647 * t17720;
    let t56726 = t5274 * t12904;
    let t56728 = t17505 * t12959;
    (t56707, t56713, t56718, t56720, t56726, t56728)
}
