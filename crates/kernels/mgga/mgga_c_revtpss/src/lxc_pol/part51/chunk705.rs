//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 705/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk705<F: Float>(t1034: F, t3182: F, t828: F, t3316: F, t994: F, t126: F, t373: F) -> (F, F, F, F) {
    let t11626 = t1034 * t1034;
    let t11627 = 1.0 / t11626;
    let t11703 = t828 * t3182;
    let t11874 = t994 * t3316;
    let t11921 = t126 * t373;
    (t11627, t11703, t11874, t11921)
}
