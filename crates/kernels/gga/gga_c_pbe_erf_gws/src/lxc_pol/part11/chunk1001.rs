//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1001/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1001<F: Float>(t3703: F, t3799: F, t6480: F, t1114: F, t346: F, t38375: F, t3863: F, t6717: F, t3916: F, t6566: F, t3867: F, t21293: F, t3841: F, param_a_c: F) -> (F, F, F, F, F, F, F) {
    let t39052 = t3703 * param_a_c;
    let t39082 = t6480 * t3799;
    let t39095 = t1114 * t38375 * t346;
    let t39174 = t6717 * t3863;
    let t39181 = t3916 * t6566;
    let t39191 = t6480 * t3867;
    let t39388 = t21293 * t3841;
    (t39052, t39082, t39095, t39174, t39181, t39191, t39388)
}
