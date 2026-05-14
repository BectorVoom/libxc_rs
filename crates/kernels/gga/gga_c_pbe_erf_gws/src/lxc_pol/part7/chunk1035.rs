//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1035/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1035<F: Float>(t2115: F, t6616: F, t2129: F, t2087: F, t2189: F, t810: F, t3140: F, t3138: F, t4386: F, t2112: F, t2079: F, t2319: F, t6466: F, t6362: F, t9630: F, t6289: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t20796 = t2115 * t6616;
    let t20797 = 35.0 / 72.0 * t20796;
    let t20798 = t2129 * t6616;
    let t20799 = 35.0 / 72.0 * t20798;
    let t20800 = t2087 * t6616;
    let t20801 = 35.0 / 72.0 * t20800;
    let t20802 = t810 * t2189;
    let t20803 = t3140 * t20802;
    let t20806 = t3138 * t4386 * t20803 / 2.0;
    let t20807 = t2112 * t2112;
    let t20808 = t2079 * t20807;
    let t20813 = t2319 * t6466;
    let t20815 = t9630 * t6362;
    let t20821 = t9630 * t6289;
    (t20797, t20799, t20801, t20802, t20803, t20806, t20807, t20808, t20813, t20815, t20821)
}
