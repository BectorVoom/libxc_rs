//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 917/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk917<F: Float>(t2557: F, t545: F, t83: F, t5156: F, t5158: F, t1508: F, t2609: F, t114: F, t557: F, t1499: F, t1639: F, t5177: F, t5179: F, t4996: F, t5005: F, t5011: F, t5019: F, t5022: F, t5154: F, t5170: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t7028 = t2557 * t545;
    let t7030 = 2.0 * t83 * t7028;
    let t7031 = 0.5848223622634646207e0 * t5156;
    let t7032 = 0.34631718211362927518e2 * t5158;
    let t7033 = t2609 * t1508;
    let t7034 = 0.17315859105681463759e2 * t7033;
    let t7035 = t2557 * t114;
    let t7037 = 0.11696447245269292414e1 * t7035 * t557;
    let t7038 = t2609 * t1499;
    let t7039 = 0.5848223622634646207e0 * t7038;
    let t7040 = t2609 * t1639;
    let t7041 = 0.11696447245269292414e1 * t7040;
    let t7042 = 12.0 * t5177;
    let t7043 = 24.0 * t5179;
    let t7044 = t7030 - t5154 - t7031 - t7032 + t4996 + t5005 - t5011 + t5170 - t7034 - t7037 - t7039 + t7041 + t5019 - t5022 - t7042 - t7043;
    (t7028, t7030, t7031, t7032, t7033, t7034, t7035, t7037, t7038, t7039, t7040, t7041, t7042, t7043, t7044)
}
