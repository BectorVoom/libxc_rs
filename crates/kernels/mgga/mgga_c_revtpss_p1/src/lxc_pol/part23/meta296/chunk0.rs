//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1538/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1538<F: Float>(t126: F, t3181: F, t1003: F, t3080: F, t221: F, t346: F, t68: F, t345: F, t1014: F, t2852: F, t245: F, t3089: F) -> (F, F, F, F, F, F) {
    let t11725 = t126 * t3181;
    let t11732 = t1003 * t3080;
    let t11735 = t221 * t68 * t346;
    let t11737 = F::cast_from(5.0_f64) / F::cast_from(1296.0_f64) * t345 * t11735;
    let t11765 = t1014 * t2852;
    let t11772 = t3089 * t245;
    (t11725, t11732, t11735, t11737, t11765, t11772)
}
