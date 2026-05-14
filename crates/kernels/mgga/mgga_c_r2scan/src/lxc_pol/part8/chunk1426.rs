//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1426/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1426<F: Float>(t1567: F, t2124: F, t22800: F, t24615: F, t24674: F, t2557: F, t25720: F, t2591: F, t26029: F, t2654: F, t27023: F, t3053: F, t30964: F, t32314: F, t32319: F, t33999: F, t34524: F, t34528: F, t360: F, t5108: F, t5109: F, t6106: F, t8029: F, t8765: F, t8792: F, t8822: F, t9148: F, t9502: F, t9534: F, t9947: F) -> (F,) {
    let t34570 = -0.15602799132097683414e1 * t6106 * t5109 * t34524 - 0.39006997830244208535e0 * t5108 * t5109 * t3053 * t2654 - 0.15602799132097683414e1 * t6106 * t5109 * t34528 - 0.52009330440325611378e0 * t25720 * t8765 - 0.2600466522016280569e0 * t8792 * t9148 - 0.26004665220162805689e0 * t26029 * t8822 - 0.78013995660488417067e0 * t24674 * t9502 + 0.54878743191129263322e-1 * t2557 * t2124 * t1567 * t9947 * t2591 - 0.98781737744032673979e0 * t2557 * t2124 * t32314 * t9534 + 0.10401866088065122276e1 * t24615 * t360 * t32319 * t33999 - 0.15602799132097683414e1 * t8029 * t360 * t32319 * t9534 - t27023 - 0.73613752582167450608e0 * t22800 + 0.2037639021386884617e0 * t30964;
    (t34570,)
}
