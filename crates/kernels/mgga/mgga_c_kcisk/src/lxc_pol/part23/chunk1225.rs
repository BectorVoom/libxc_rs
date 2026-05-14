//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1225/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1225<F: Float>(t33652: F, t6370: F, t32277: F, t491: F, t6323: F, t5967: F, t6332: F, t9491: F, t32269: F, t9836: F, t1493: F, t2258: F, t1340: F, t6363: F, t1415: F, t6340: F) -> (F, F, F, F, F, F, F, F, F) {
    let t33653 = t33652 * t6370;
    let t33655 = t491 * t32277;
    let t33656 = t33655 * t6323;
    let t33658 = t6332 * t5967;
    let t33659 = t9491 * t33658;
    let t33661 = t32269 * t9836;
    let t33663 = t2258 * t1493;
    let t33665 = t1340 * t6363;
    let t33667 = t1415 * t6340;
    (t33653, t33655, t33656, t33658, t33659, t33661, t33663, t33665, t33667)
}
