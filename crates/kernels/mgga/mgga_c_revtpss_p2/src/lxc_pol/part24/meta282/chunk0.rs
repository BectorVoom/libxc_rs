//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1058/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1058<F: Float>(t2852: F, t5825: F, t11354: F, t6113: F, t11358: F, t6132: F, t698: F, t6135: F, t6138: F, t300: F, t6184: F, t6104: F, t914: F) -> (F, F, F, F, F, F, F, F) {
    let t18936 = t2852 * t5825;
    let t18979 = t11354 * t6113;
    let t18987 = t11358 * t6113;
    let t19002 = t698 * t6132;
    let t19004 = t698 * t6135;
    let t19009 = t698 * t6138;
    let t19049 = t300 * t6184;
    let t19056 = t6104 * t914;
    (t18936, t18979, t18987, t19002, t19004, t19009, t19049, t19056)
}
