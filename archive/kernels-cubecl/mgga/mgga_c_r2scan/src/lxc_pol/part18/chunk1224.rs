//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1224/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1224<F: Float>(t11686: F, t11744: F, t12550: F, t2201: F, t3324: F, t11760: F, t11770: F, t12459: F, t3336: F, t5095: F, t2892: F, t3319: F, t3320: F) -> (F, F, F, F, F) {
    let t43602 = t11744 * t11686;
    let t43606 = t2201 * t12550 * t3324;
    let t43609 = t2201 * t11760 * t11770;
    let t43612 = t5095 * t3336 * t12459;
    let t43616 = t5095 * t3319 * t3320 * t2892;
    (t43602, t43606, t43609, t43612, t43616)
}
