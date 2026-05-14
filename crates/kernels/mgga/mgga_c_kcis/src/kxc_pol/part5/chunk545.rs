//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 545/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk545<F: Float>(t3160: F, t829: F, t1135: F, t2861: F, t1085: F, t1094: F, t1130: F, t982: F, t89: F, t828: F, t1018: F, t341: F, t1017: F, t86: F, sigma0: F) -> (F, F, F, F, F, F, F, F, F) {
    let t3161 = t3160 * t829;
    let t3174 = t2861 * t1135;
    let t3177 = t1085 * t1094;
    let t3178 = t3177 * sigma0;
    let t3182 = t982 * t1130;
    let t3187 = 2.0 * t89;
    let t3188 = 2.0 * t828;
    let t3198 = t1018 * t341;
    let t3200 = t86 * t1017 * t3198;
    (t3161, t3174, t3177, t3178, t3182, t3187, t3188, t3198, t3200)
}
