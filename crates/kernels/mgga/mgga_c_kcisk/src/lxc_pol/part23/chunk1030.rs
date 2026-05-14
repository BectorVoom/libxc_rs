//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1030/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1030<F: Float>(t1391: F, t5626: F, t14090: F, t1049: F, t3532: F, t5854: F, t970: F, t1398: F, t19127: F, t1375: F, t19132: F, t19119: F, t457: F, t1383: F, t1186: F, t19114: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t20670 = 0.47822877300252710492e-1 * t1391 * t5626;
    let t20676 = 0.62154466893555682512e-3 * t14090 * t5626;
    let t20679 = t1049 * t3532;
    let t20687 = 0.4705225e-4 * t970 * t5854;
    let t20688 = t1398 * t19127;
    let t20691 = t1375 * t19132;
    let t20694 = t457 * t19119;
    let t20697 = t1383 * t19132;
    let t20700 = t1186 * t19119;
    let t20703 = t1398 * t19132;
    let t20706 = t1375 * t19114;
    (t20670, t20676, t20679, t20687, t20688, t20691, t20694, t20697, t20700, t20703, t20706)
}
