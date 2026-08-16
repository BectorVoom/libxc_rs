//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1214/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1214<F: Float>(t11822: F, t22688: F, t1012: F, t11827: F, t23481: F, t247: F, t3182: F, t1592: F, t19675: F, t1042: F, t11660: F, t1469: F) -> (F, F, F, F, F, F, F, F) {
    let t23873 = t11822 * t22688;
    let t23874 = t1012 * t23873;
    let t23877 = t11827 * t22688;
    let t23878 = t1012 * t23877;
    let t23886 = t247 * t3182 * t23481;
    let t23891 = t19675 * t1592;
    let t23892 = t1042 * t23891;
    let t23898 = t11660 * t1469;
    (t23873, t23874, t23877, t23878, t23886, t23891, t23892, t23898)
}
