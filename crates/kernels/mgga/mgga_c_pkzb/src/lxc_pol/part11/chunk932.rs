//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 932/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk932<F: Float>(t3849: F, t904: F, t3806: F, t6121: F, t3161: F, t898: F, t2328: F, t3841: F, t3833: F, t881: F, t890: F, t9929: F) -> (F, F, F, F, F, F, F) {
    let t10148 = t3849 * t904;
    let t10150 = t6121 * t3806;
    let t10151 = t10150 * t3161;
    let t10153 = F::cast_from(0.10389515463408878255e3_f64) * t898 * t10151;
    let t10155 = F::cast_from(0.17315859105681463759e2_f64) * t2328 * t3841;
    let t10157 = F::cast_from(0.11696447245269292414e1_f64) * t2328 * t3833;
    let t10159 = t881 * t9929 * t890;
    (t10148, t10150, t10151, t10153, t10155, t10157, t10159)
}
