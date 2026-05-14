//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 660/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk660<F: Float>(t825: F, t9847: F, t1114: F, t3047: F, t3083: F, t3052: F, t3724: F, t840: F, t1161: F, t8589: F, t829: F, t830: F) -> (F, F, F, F, F, F) {
    let t9848 = t9847 * t825;
    let t9849 = t1114 * t9848;
    let t9852 = t3083 * t3047;
    let t9854 = t3083 * t3052;
    let t9879 = t840 * t3724;
    let t9883 = t8589 * t1161;
    let t9885 = t829 * t830 * t9883;
    (t9848, t9849, t9852, t9854, t9879, t9885)
}
