//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1127/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1127<F: Float>(t1365: F, t7054: F, t531: F, t7086: F, t833: F, t3984: F, t1380: F, t6944: F, t1444: F, t6284: F) -> (F, F, F, F) {
    let t21061 = t7054 * t1365;
    let t21063 = t7086 * t531;
    let t21064 = t21063 * t833;
    let t21065 = t3984 * t21064;
    let t21068 = t6944 * t1380;
    let t21069 = t3984 * t21068;
    let t21072 = t1444 * t6284;
    let t21073 = t21072 * t833;
    (t21061, t21065, t21069, t21073)
}
