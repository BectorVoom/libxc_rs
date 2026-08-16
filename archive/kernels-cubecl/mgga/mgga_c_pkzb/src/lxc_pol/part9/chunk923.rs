//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 923/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk923<F: Float>(t2557: F, t545: F, t83: F, t5156: F, t5158: F, t1508: F, t2609: F, t114: F, t557: F, t1499: F, t1639: F, t5177: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t7028 = t2557 * t545;
    let t7030 = F::cast_from(2.0_f64) * t83 * t7028;
    let t7031 = F::cast_from(0.5848223622634646207e0_f64) * t5156;
    let t7032 = F::cast_from(0.34631718211362927518e2_f64) * t5158;
    let t7033 = t2609 * t1508;
    let t7034 = F::cast_from(0.17315859105681463759e2_f64) * t7033;
    let t7035 = t2557 * t114;
    let t7037 = F::cast_from(0.11696447245269292414e1_f64) * t7035 * t557;
    let t7038 = t2609 * t1499;
    let t7039 = F::cast_from(0.5848223622634646207e0_f64) * t7038;
    let t7040 = t2609 * t1639;
    let t7041 = F::cast_from(0.11696447245269292414e1_f64) * t7040;
    let t7042 = F::cast_from(12.0_f64) * t5177;
    (t7028, t7030, t7031, t7032, t7034, t7035, t7037, t7039, t7041, t7042)
}
