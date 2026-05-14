//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1241/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1241<F: Float>(t2799: F, t8968: F, t11701: F, t2594: F, t9988: F, t5218: F, t9094: F, t2580: F, t6974: F, t2587: F, t6719: F, t2591: F, t1873: F, t9069: F, t33121: F, t9072: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t35274 = t2799 * t8968;
    let t35276 = 6.0 * t11701 * t35274;
    let t35277 = t9988 * t2594;
    let t35279 = 4.0 * t5218 * t35277;
    let t35280 = t2799 * t9094;
    let t35282 = 2.0 * t5218 * t35280;
    let t35283 = t6974 * t2580;
    let t35285 = t6719 * t2587;
    let t35287 = t6719 * t2591;
    let t35289 = t1873 * t9069;
    let t35291 = t33121 * t9072;
    (t35274, t35276, t35277, t35279, t35280, t35282, t35283, t35285, t35287, t35289, t35291)
}
