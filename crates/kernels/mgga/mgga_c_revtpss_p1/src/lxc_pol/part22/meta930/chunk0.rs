//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3157/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3157<F: Float>(t12772: F, t17673: F, t3625: F, t12910: F, t12916: F, t17460: F, t17213: F, t3172: F, t5384: F, t13069: F, t5265: F, t1260: F, t17332: F) -> (F, F, F, F, F) {
    let t57170 = t3625 * t12772 * t17673;
    let t57173 = t12910 * t12916 * t17460;
    let t57176 = t5384 * t3172 * t17213;
    let t57178 = t13069 * t5265;
    let t57187 = t17332 * t1260;
    (t57170, t57173, t57176, t57178, t57187)
}
