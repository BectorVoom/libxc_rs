//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1021/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1021<F: Float>(t140: F, t3252: F, t1012: F, t11821: F, t12047: F, t15905: F, t12167: F, t3057: F, t380: F, t3088: F, t370: F, t994: F) -> (F, F, F, F, F, F, F) {
    let t15993 = t140 * t3252;
    let t16012 = t1012 * t11821;
    let t16067 = t12047 * t15905;
    let t16081 = t12167 * t15905;
    let t16087 = t3057 * t380;
    let t16088 = t3088 * t370;
    let t16089 = t16087 * t16088;
    let t16094 = t994 * t380;
    (t15993, t16012, t16067, t16081, t16088, t16089, t16094)
}
