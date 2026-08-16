//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 580/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk580<F: Float>(t3147: F, t900: F, t1217: F, t2328: F, t1208: F, t2295: F, t891: F, t898: F, t3135: F, t881: F, t890: F, t2317: F) -> (F, F, F, F, F, F, F, F) {
    let t3149 = F::cast_from(0.5848223622634646207e0_f64) * t3147 * t900;
    let t3151 = F::cast_from(0.5848223622634646207e0_f64) * t2328 * t1217;
    let t3152 = t2295 * t1208;
    let t3153 = t3152 * t891;
    let t3155 = F::cast_from(0.11696447245269292414e1_f64) * t898 * t3153;
    let t3157 = t881 * t3135 * t890;
    let t3159 = F::cast_from(0.5848223622634646207e0_f64) * t898 * t3157;
    let t3160 = t2317 * t1208;
    (t3149, t3151, t3152, t3153, t3155, t3157, t3159, t3160)
}
