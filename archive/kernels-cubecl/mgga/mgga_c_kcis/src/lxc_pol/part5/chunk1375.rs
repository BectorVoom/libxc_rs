//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1375/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1375<F: Float>(t22640: F, t4292: F, t2062: F, t6020: F, t6016: F, t6038: F, t6044: F, t21799: F, t6011: F, t17463: F, t2061: F, t5928: F) -> (F, F, F, F, F, F) {
    let t22641 = t4292 * t22640;
    let t22643 = t6020 * t2062;
    let t22645 = t6016 * t6038;
    let t22647 = t6016 * t6044;
    let t22649 = t6011 * t21799;
    let t22650 = t17463 * t22649;
    let t22652 = t2061 * t5928;
    (t22641, t22643, t22645, t22647, t22650, t22652)
}
