//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 888/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk888<F: Float>(t2231: F, t382: F, t5857: F, t970: F, t1391: F, t5626: F, t14090: F, t1049: F, t3532: F, t5854: F, t5860: F, t960: F, t5845: F, t5848: F, t965: F, t5851: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t20634 = t382 * t2231;
    let t20660 = t970 * t5857;
    let t20670 = 0.47822877300252710492e-1 * t1391 * t5626;
    let t20676 = 0.62154466893555682512e-3 * t14090 * t5626;
    let t20679 = t1049 * t3532;
    let t20687 = 0.4705225e-4 * t970 * t5854;
    let t20718 = 0.18736e-1 * t960 * t5860;
    let t20719 = t960 * t5845;
    let t20736 = t965 * t5848;
    let t20739 = 0.17611111111111111111e-2 * t965 * t5851;
    (t20634, t20660, t20670, t20676, t20679, t20687, t20718, t20719, t20736, t20739)
}
