//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 961/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk961<F: Float>(t7315: F, t7536: F, t25089: F, t7488: F, t2107: F, t25802: F, t1310: F, t7373: F, t116: F, t7356: F) -> (F, F, F, F, F) {
    let t26380 = t7536 * t7315;
    let t26383 = t7488 * t25089;
    let t26392 = t2107 * t25802;
    let t26396 = t1310 * t7373;
    let t26399 = t7356 * t116;
    (t26380, t26383, t26392, t26396, t26399)
}
