//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 661/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk661<F: Float>(t1096: F, t5019: F, t1092: F, t1775: F, t2861: F, t1094: F, t1747: F, sigma0: F) -> (F, F, F, F, F) {
    let t5020 = t1096 * t5019;
    let t5021 = t1092 * t5020;
    let t5023 = t2861 * t1775;
    let t5025 = t1747 * t1094;
    let t5026 = t5025 * sigma0;
    (t5020, t5021, t5023, t5025, t5026)
}
