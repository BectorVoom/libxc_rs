//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 917/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk917<F: Float>(t7109: F, t7111: F, t3037: F, t406: F, t410: F, t7127: F, t5025: F, t5027: F, t5029: F, t7157: F, t7159: F, t5034: F, t4873: F, t5039: F, t7097: F, t7126: F, t7156: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t8646 = 40.0 * t7109;
    let t8647 = 24.0 * t7111;
    let t8648 = t406 * t3037;
    let t8649 = 4.0 * t8648;
    let t8650 = t410 * t3037;
    let t8651 = 4.0 * t8650;
    let t8652 = 0.23392894490538584828e1 * t7127;
    let t8653 = 8.0 * t5025;
    let t8654 = 8.0 * t5027;
    let t8655 = 0.5848223622634646207e0 * t5029;
    let t8656 = 0.11696447245269292414e1 * t7157;
    let t8657 = 0.34631718211362927517e2 * t7159;
    let t8658 = 0.11696447245269292414e1 * t5034;
    let t8659 = -t7097 + t8646 - t8647 + t8649 - t8651 + t7126 + t8652 - t8653 - t8654 - t8655 + t4873 - t7156 - t8656 - t8657 + t8658 + t5039;
    (t8646, t8647, t8648, t8649, t8650, t8651, t8652, t8653, t8654, t8655, t8656, t8657, t8658, t8659)
}
