//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2863/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2863<F: Float>(t11262: F, t3600: F, t3605: F, t3617: F, t675: F, t1261: F, t247: F, t3363: F, t3609: F, t44169: F, t1263: F, t215: F) -> (F, F, F, F, F) {
    let t44675 = t3600 * t11262 * t3605;
    let t44693 = t675 * t3617;
    let t44696 = t1261 * t247 * t44693 * t3363;
    let t44698 = t44169 * t3609;
    let t44701 = t215 * t1263;
    (t44675, t44693, t44696, t44698, t44701)
}
