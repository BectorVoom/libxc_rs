//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1222/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1222<F: Float>(t6016: F, t6038: F, t6044: F, t21799: F, t6011: F, t17463: F, t2061: F, t5928: F, t1546: F, t4281: F, t7305: F, t22471: F, t22632: F, t22634: F, t22638: F, t22641: F, t22643: F) -> (F, F, F, F, F, F) {
    let t22645 = t6016 * t6038;
    let t22647 = t6016 * t6044;
    let t22649 = t6011 * t21799;
    let t22650 = t17463 * t22649;
    let t22652 = t2061 * t5928;
    let t22653 = t1546 * t22652;
    let t22655 = t4281 * t7305;
    let t22657 = -t22471 / 576.0 + t22632 / 16.0 + t22634 / 8.0 - t22638 / 256.0 + t22641 / 192.0 + t22643 / 24.0 - t22645 / 8.0 + t22647 / 3.0 + 3.0 / 128.0 * t22650 - t22653 / 24.0 + t22655 / 256.0;
    (t22645, t22647, t22650, t22653, t22655, t22657)
}
