//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1066/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1066<F: Float>(t2370: F, t46654: F, t830: F, t1114: F, t44900: F, t825: F, t3083: F, t9820: F, t12138: F, t35277: F, t3733: F, t2501: F, t3703: F) -> (F, F, F, F, F, F) {
    let t46656 = t2370 * t830 * t46654;
    let t46667 = t1114 * t44900 * t825;
    let t46678 = t3083 * t9820;
    let t46685 = t3083 * t12138;
    let t46703 = t35277 * t3733;
    let t46705 = t2501 * t3703;
    (t46656, t46667, t46678, t46685, t46703, t46705)
}
