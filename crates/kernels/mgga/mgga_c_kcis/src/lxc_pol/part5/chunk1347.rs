//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1347/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1347<F: Float>(t1396: F, t22252: F, t4123: F, t1464: F, t15955: F, t5671: F, t12241: F, t15909: F, t3728: F, t6919: F, t17292: F, t5663: F) -> (F, F, F, F) {
    let t22253 = t1396 * t22252;
    let t22254 = t4123 * t22253;
    let t22255 = t1464 * t22254;
    let t22259 = t15955 * t5671;
    let t22260 = t12241 * t22259;
    let t22261 = t15909 * t22260;
    let t22263 = t3728 * t6919;
    let t22265 = t17292 * t5663;
    (t22255, t22261, t22263, t22265)
}
