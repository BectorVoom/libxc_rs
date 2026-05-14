//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1040/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1040<F: Float>(t31834: F, t33722: F, t14691: F, t246: F, t31851: F, t8486: F, t120042: F, t1549: F, t31827: F, t31831: F, t31755: F, t31756: F, t4364: F, t4424: F, t25317: F, t4486: F) -> (F, F, F, F, F, F) {
    let t126390 = t31834 * t33722;
    let t126394 = t8486 * t31851 * t246 * t14691;
    let t126396 = t120042 * t1549;
    let t126397 = t31827 * t126396;
    let t126399 = t31831 * t126396;
    let t126403 = t31755 * t4364 * t31756 * t4424;
    let t126405 = t25317 * t4486;
    (t126390, t126394, t126397, t126399, t126403, t126405)
}
