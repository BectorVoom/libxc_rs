//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1255/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1255<F: Float>(t27805: F, t45306: F, t12065: F, t46327: F, t45753: F, t45767: F, t2157: F, t49847: F, t2155: F, t858: F, t867: F, t3180: F, t46199: F) -> (F, F, F, F, F, F, F) {
    let t49921 = t27805 * t45306 / F::new(4.0);
    let t49928 = F::new(7.0) / F::new(48.0) * t46327 * t12065;
    let t49929 = F::new(7.0) / F::new(24.0) * t45753;
    let t49931 = F::new(7.0) / F::new(24.0) * t45767;
    let t49932 = t49847 * t2157;
    let t49936 = F::new(7.0) / F::new(48.0) * t2155 * t867 * t858 * t49932;
    let t49943 = t46199 * t3180 / F::new(6.0);
    (t49921, t49928, t49929, t49931, t49932, t49936, t49943)
}
