//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2024/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2024<F: Float>(t10868: F, t241: F, t820: F, t14547: F, t4364: F, t4365: F, t2724: F, t2747: F, t4450: F, t14676: F, t4366: F, t10811: F, t4452: F) -> (F, F, F, F, F) {
    let t14894 = t820 * t10868 * t241;
    let t14896 = t4364 * t4365 * t14547;
    let t14900 = t2747 * t4450 * t2724;
    let t14904 = t4364 * t14676 * t4366;
    let t14907 = t10811 * t4452;
    (t14894, t14896, t14900, t14904, t14907)
}
