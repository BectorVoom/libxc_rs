//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3007/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3007<F: Float>(t10696: F, t1544: F, t14832: F, t2394: F, t2661: F, t14668: F, t14923: F, t124: F, t4423: F, t14686: F, t14931: F, t4366: F) -> (F, F, F, F) {
    let t50396 = t10696 * t1544;
    let t50399 = t2661 * t14832 * t50396 * t2394;
    let t50409 = t14923 * t14668;
    let t50412 = t124 * t4423;
    let t50415 = t14931 * t14686 * t50412 * t4366;
    (t50399, t50409, t50412, t50415)
}
