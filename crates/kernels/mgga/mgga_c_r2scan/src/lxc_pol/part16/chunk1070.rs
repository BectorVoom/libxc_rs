//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1070/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1070<F: Float>(t38055: F, t40042: F, t40044: F, t40048: F, t40051: F, t40054: F, t40077: F, t40087: F, t40090: F, t41680: F, t43488: F, t43490: F, t12455: F, t3336: F, t5103: F, t11659: F, t7601: F) -> (F, F, F) {
    let t43493 = t40042 + 0.13972381860938637374e0 * t40044 + t40048 + t40051 - t40054 + 0.17336443480108537126e0 * t43488 - t41680 + t40077 - t38055 + 0.97574405393827830187e-2 * t43490 - t40087 + 0.55889527443754549496e0 * t40090;
    let t43495 = t5103 * t3336 * t12455;
    let t43497 = t7601 * t11659;
    (t43493, t43495, t43497)
}
