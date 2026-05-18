//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1331/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1331<F: Float>(t1419: F, t3766: F, t6912: F, t21106: F, t5439: F, t21110: F, t1319: F, t3761: F, t6944: F, t21073: F, t1482: F, t21585: F) -> (F, F, F, F, F, F, F) {
    let t22015 = t3766 * t6912 * t1419;
    let t22018 = t5439 * t21106;
    let t22021 = t5439 * t21110;
    let t22025 = t3761 * t6944 * t1319;
    let t22029 = t3766 * t6944 * t1419;
    let t22032 = t5439 * t21073;
    let t22035 = t1482 * t21585;
    (t22015, t22018, t22021, t22025, t22029, t22032, t22035)
}
