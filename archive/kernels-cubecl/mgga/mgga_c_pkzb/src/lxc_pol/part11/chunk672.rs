//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 672/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk672<F: Float>(t237: F, t3826: F, t3802: F, t1217: F, t3147: F, t2295: F, t3806: F, t890: F, t898: F, t3819: F, t881: F, t2317: F) -> (F, F, F, F, F, F, F, F) {
    let t3827 = t237 * t3826;
    let t3829 = F::cast_from(0.19751673498613801407e-1_f64) * t237 * t3802;
    let t3831 = F::cast_from(0.11696447245269292414e1_f64) * t3147 * t1217;
    let t3833 = t2295 * t3806 * t890;
    let t3835 = F::cast_from(0.11696447245269292414e1_f64) * t898 * t3833;
    let t3837 = t881 * t3819 * t890;
    let t3839 = F::cast_from(0.5848223622634646207e0_f64) * t898 * t3837;
    let t3840 = t2317 * t3806;
    (t3827, t3829, t3831, t3833, t3835, t3837, t3839, t3840)
}
