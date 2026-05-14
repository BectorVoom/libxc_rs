//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1049/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1049<F: Float>(t10868: F, t7614: F, t7615: F, t11714: F, t6493: F, t1592: F, t27177: F, t3308: F, t11780: F, t2201: F, t3324: F, t10826: F, t25983: F, t39885: F, t8243: F, t2605: F, t37699: F) -> (F, F, F, F, F, F, F) {
    let t40090 = t7614 * t10868 * t7615;
    let t40092 = t6493 * t11714;
    let t40095 = t1592 * t3308 * t27177;
    let t40098 = t2201 * t11780 * t3324;
    let t40100 = t25983 * t10826;
    let t40102 = t39885 * t8243;
    let t40103 = 0.19514881078765566037e-1 * t40102;
    let t40107 = t37699 * t2605;
    (t40090, t40092, t40095, t40098, t40100, t40103, t40107)
}
