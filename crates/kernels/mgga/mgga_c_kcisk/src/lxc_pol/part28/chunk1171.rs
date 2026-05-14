//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1171/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1171<F: Float>(t34097: F, t9664: F, t32903: F, t9945: F, t1799: F, t220: F, t5185: F, t9679: F, t6713: F, t2575: F, t654: F) -> (F, F, F, F, F, F, F) {
    let t34098 = t9664 * t34097;
    let t34100 = t32903 * t9945;
    let t34101 = t1799 * t34100;
    let t34103 = t5185 * t220;
    let t34104 = t9679 * t34103;
    let t34105 = t6713 * t34104;
    let t34107 = t2575 * t654;
    (t34098, t34100, t34101, t34103, t34104, t34105, t34107)
}
