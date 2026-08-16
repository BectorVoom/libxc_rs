//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3027/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3027<F: Float>(t14923: F, t14927: F, t10811: F, t14697: F, t40672: F, t828: F, t10905: F, t14825: F, t14829: F, t14819: F, t40517: F, t14910: F) -> (F, F, F, F, F, F, F) {
    let t51000 = t14923 * t14927;
    let t51006 = t10811 * t14697;
    let t51014 = t40672 * t828;
    let t51026 = t10905 * t14825;
    let t51028 = t10905 * t14829;
    let t51042 = t40517 * t14819;
    let t51047 = t10811 * t14910;
    (t51000, t51006, t51014, t51026, t51028, t51042, t51047)
}
