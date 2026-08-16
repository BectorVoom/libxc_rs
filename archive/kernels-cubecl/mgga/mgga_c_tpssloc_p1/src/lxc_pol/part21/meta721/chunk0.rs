//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2565/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2565<F: Float>(t14536: F, t225: F, t10164: F, t1634: F, t14532: F, t14562: F, t14527: F, t14534: F, t11190: F, t1670: F, t3242: F, t457: F) -> (F, F, F, F, F, F, F, F) {
    let t50625 = t14536 * t225;
    let t50628 = t10164 * t1634;
    let t50632 = t14532 * t225;
    let t50653 = t14562 * t225;
    let t50690 = t14527 * t225;
    let t50703 = t14534 * t225;
    let t50819 = t11190 * t1670;
    let t50822 = t457 * t3242;
    (t50625, t50628, t50632, t50653, t50690, t50703, t50819, t50822)
}
