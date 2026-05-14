//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 820/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk820<F: Float>(t5024: F, t5522: F, t639: F, t661: F, t1648: F, t4924: F, t1740: F, t1775: F, t5502: F, t7011: F, t16712: F, t197: F, t16669: F, t5293: F, t587: F, t1820: F, t5018: F, t5300: F) -> (F, F, F, F, F, F) {
    let t16925 = 32.0 / 9.0 * t639 * t5522 * t5024 * t661;
    let t16927 = 32.0 / 15.0 * t1648 * t4924;
    let t16928 = t1775 * t1740;
    let t16929 = 16.0 / 15.0 * t16928;
    let t16931 = 16.0 / 5.0 * t7011 * t5502;
    let t16932 = t197 * t16712;
    let t16936 = 128.0 / 27.0 * t587 * t5293 * t16932 * t16669;
    let t16938 = t1820 * t5018 * t5300;
    (t16925, t16927, t16929, t16931, t16936, t16938)
}
