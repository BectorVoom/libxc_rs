//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2501/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2501<F: Float>(t12861: F, t12916: F, t3718: F, t11262: F, t3600: F, t3605: F, t1261: F, t12925: F, t3172: F, t12921: F, t3711: F, t3617: F, t675: F) -> (F, F, F, F, F) {
    let t44672 = t3718 * t12916 * t12861;
    let t44675 = t3600 * t11262 * t3605;
    let t44678 = t1261 * t3172 * t12925;
    let t44681 = t3711 * t3172 * t12921;
    let t44693 = t675 * t3617;
    (t44672, t44675, t44678, t44681, t44693)
}
