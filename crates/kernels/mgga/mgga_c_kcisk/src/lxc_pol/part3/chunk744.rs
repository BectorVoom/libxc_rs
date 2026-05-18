//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 744/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk744<F: Float>(t5094: F, t696: F, t1806: F, t5102: F, t143: F, t4597: F, t10441: F, t682: F, t1814: F, t3290: F, t1824: F, t1810: F, t3293: F) -> (F, F, F, F, F, F) {
    let t11491 = t696 * t5094;
    let t11493 = t1806 * t5102;
    let t11495 = t143 * t4597;
    let t11496 = t682 * t10441;
    let t11499 = t1814 * t3290;
    let t11500 = t11499 * t1824;
    let t11503 = t1810 * t3293;
    (t11491, t11493, t11495, t11496, t11500, t11503)
}
