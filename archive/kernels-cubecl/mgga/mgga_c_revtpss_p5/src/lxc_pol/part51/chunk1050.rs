//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1050/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1050<F: Float>(t11921: F, t247: F, t31886: F, t8502: F, t31908: F, t31991: F, t1039: F, t31997: F, t31998: F, t72: F, t120334: F, t1982: F) -> (F, F, F, F) {
    let t120385 = t8502 * t247 * t11921 * t31886;
    let t120387 = t31908 * t31991;
    let t120397 = t31997 * t31998 * t1039 * t72;
    let t120400 = t1982 * t120334;
    (t120385, t120387, t120397, t120400)
}
