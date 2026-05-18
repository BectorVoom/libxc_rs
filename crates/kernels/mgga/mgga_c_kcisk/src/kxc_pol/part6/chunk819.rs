//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 819/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk819<F: Float>(t1173: F, t7894: F, t1171: F, t7748: F, t3722: F, t7819: F, t7779: F, t827: F, t7782: F, t7776: F, t45: F, t7796: F) -> (F, F, F, F, F, F, F) {
    let t25623 = t1173 * t7894;
    let t25663 = t7748 * t1171;
    let t25668 = t3722 * t7819;
    let t25696 = t827 * t7779;
    let t25699 = t827 * t7782;
    let t25701 = t827 * t7776;
    let t25786 = t45 * t7796;
    (t25623, t25663, t25668, t25696, t25699, t25701, t25786)
}
