//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 869/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk869<F: Float>(t1962: F, t833: F, t1961: F, t4035: F, t1419: F, t1409: F, t5526: F, t1650: F, t532: F, t4061: F, t1444: F, t822: F) -> (F, F, F, F, F, F, F) {
    let t5801 = t1962 * t833;
    let t5804 = t4035 * t1961;
    let t5805 = t5804 * t1419;
    let t5808 = t1409 * t5526;
    let t5814 = t532 * t1650;
    let t5816 = t4061 * t1650;
    let t5820 = t822 * t1444;
    (t5801, t5804, t5805, t5808, t5814, t5816, t5820)
}
