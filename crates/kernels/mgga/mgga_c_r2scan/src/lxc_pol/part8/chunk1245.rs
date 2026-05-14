//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1245/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1245<F: Float>(t7494: F, t9160: F, t2139: F, t3100: F, t6848: F, t9517: F, t259: F, t2665: F, t6448: F, t9156: F, t9540: F, t6118: F, t9254: F, t2294: F, t6106: F, t8784: F) -> (F, F, F, F, F, F, F, F, F) {
    let t27746 = t7494 * t9160;
    let t27753 = t2139 * t6848 * t3100;
    let t27763 = t7494 * t9517;
    let t27774 = t2665 * t259;
    let t27775 = t6448 * t27774;
    let t27786 = t7494 * t9156;
    let t27814 = t7494 * t9540;
    let t27820 = t6118 * t9254;
    let t27823 = t6106 * t2294 * t8784;
    (t27746, t27753, t27763, t27774, t27775, t27786, t27814, t27820, t27823)
}
