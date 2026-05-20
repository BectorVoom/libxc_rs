//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2145/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2145<F: Float>(t1868: F, t4003: F, t22046: F, t3936: F, t124: F, t22809: F, t800: F, t6816: F) -> (F, F, F, F, F) {
    let t22841 = t4003 * t1868;
    let t22843 = t3936 * t22046 * t22841;
    let t22848 = t124 * t22809;
    let t22849 = t800 * t22848;
    let t22852 = t6816 * t1868;
    (t22841, t22843, t22848, t22849, t22852)
}
