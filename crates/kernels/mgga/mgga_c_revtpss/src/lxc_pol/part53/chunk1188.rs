//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1188/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1188<F: Float>(t120082: F, t33716: F, t119935: F, t33674: F, t31834: F, t33722: F, t14691: F, t246: F, t31851: F, t8486: F, t120042: F, t1549: F) -> (F, F, F, F, F) {
    let t126386 = t120082 * t33716;
    let t126388 = t119935 * t33674;
    let t126390 = t31834 * t33722;
    let t126394 = t8486 * t31851 * t246 * t14691;
    let t126396 = t120042 * t1549;
    (t126386, t126388, t126390, t126394, t126396)
}
