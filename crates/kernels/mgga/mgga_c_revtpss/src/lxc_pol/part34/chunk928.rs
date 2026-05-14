//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 928/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk928<F: Float>(t11822: F, t22688: F, t1012: F, t11827: F, t23481: F, t247: F, t3182: F, t1592: F, t19675: F, t1042: F, t11660: F, t1469: F, t19501: F, t3092: F, t6266: F, t19611: F) -> (F, F, F, F, F, F, F) {
    let t23873 = t11822 * t22688;
    let t23874 = t1012 * t23873;
    let t23877 = t11827 * t22688;
    let t23878 = t1012 * t23877;
    let t23886 = t247 * t3182 * t23481;
    let t23891 = t19675 * t1592;
    let t23892 = t1042 * t23891;
    let t23898 = t11660 * t1469;
    let t23899 = t19501 * t23898;
    let t23900 = t3092 * t23899;
    let t23903 = t19501 * t6266;
    let t23904 = t3092 * t23903;
    let t23907 = t19611 * t6266;
    (t23874, t23878, t23886, t23892, t23900, t23904, t23907)
}
