//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 837/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk837<F: Float>(t17166: F, t1406: F, t1820: F, t1885: F, t5299: F, t5292: F, t9: F, t5295: F, t587: F, t16746: F, t590: F, t591: F, t1663: F, t187: F, t22: F, t16740: F, t197: F) -> (F, F, F, F, F, F) {
    let t17167 = 8.0 / 45.0 * t17166;
    let t17171 = 16.0 / 5.0 * t1820 * t1885 * t5299 * t1406;
    let t17172 = t9 * t5292;
    let t17174 = t587 * t17172 * t5295;
    let t17175 = 256.0 / 243.0 * t17174;
    let t17179 = 4.0 / 45.0 * t587 * t590 * t591 * t16746;
    let t17182 = t22 / t187 / t1663;
    let t17183 = t197 * t16740;
    (t17167, t17171, t17175, t17179, t17182, t17183)
}
