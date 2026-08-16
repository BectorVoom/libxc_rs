//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 743/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk743<F: Float>(t4171: F, t5648: F, t4170: F, t4160: F, t1444: F, t556: F, t1650: F, t833: F) -> (F, F, F, F, F) {
    let t5649 = t4171 * t5648;
    let t5650 = t4170 * t5649;
    let t5651 = t4160 * t5650;
    let t5653 = t556 * t1444;
    let t5654 = t1650 * t833;
    (t5649, t5650, t5651, t5653, t5654)
}
