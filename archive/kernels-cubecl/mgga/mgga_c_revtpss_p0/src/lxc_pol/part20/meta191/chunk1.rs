//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 950/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk950<F: Float>(t4003: F, t9898: F, t1390: F, t828: F, t4000: F, t820: F, t843: F, t4006: F, t136: F, t4011: F, t221: F, t3829: F) -> (F, F, F, F, F, F) {
    let t9912 = t9898 * t4003;
    let t9914 = t1390 * t828 * t9912;
    let t9918 = t820 * t4000 * t843;
    let t9919 = t9918 * t4006;
    let t9921 = t4011 * t136;
    let t9923 = t9921 * t221 * t3829;
    (t9912, t9914, t9918, t9919, t9921, t9923)
}
