//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2938/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2938<F: Float>(t13790: F, t4056: F, t10022: F, t2782: F, t10073: F, t14231: F, t10139: F, t14219: F, t9285: F, t14215: F, t2470: F, t4101: F) -> (F, F, F, F) {
    let t48025 = t13790 * t4056;
    let t48027 = t2782 * t10022 * t48025;
    let t48029 = t10073 * t14231;
    let t48036 = t10139 * t14219 * t9285;
    let t48039 = t4101 * t14215 * t2470;
    (t48027, t48029, t48036, t48039)
}
