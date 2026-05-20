//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2660/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2660<F: Float>(t20112: F, t994: F, t4746: F, t4930: F, t19855: F, t993: F, t378: F, t15654: F, t1678: F, t225: F, t11249: F, t6299: F) -> (F, F, F, F, F, F) {
    let t64737 = t994 * t20112;
    let t64764 = t4746 * t4930;
    let t64816 = t19855 * t993;
    let t64817 = t64816 * t378;
    let t64845 = t15654 * t1678;
    let t64907 = t64816 * t225;
    let t65144 = t6299 * t11249;
    (t64737, t64764, t64817, t64845, t64907, t65144)
}
