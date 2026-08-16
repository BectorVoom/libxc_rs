//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2736/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2736<F: Float>(t17241: F, t5373: F, t17654: F, t20766: F, t56756: F, t17693: F, t20937: F, t1222: F, t17240: F, t20310: F, t20306: F, t12772: F, t21156: F, t3625: F) -> (F, F, F, F, F, F) {
    let t71320 = t5373 * t17241;
    let t71329 = t17654 * t56756 * t20766;
    let t71341 = t17693 * t56756 * t20937;
    let t71373 = t1222 * t17240 * t20310;
    let t71377 = t1222 * t17240 * t20306;
    let t71400 = t3625 * t12772 * t21156;
    (t71320, t71329, t71341, t71373, t71377, t71400)
}
