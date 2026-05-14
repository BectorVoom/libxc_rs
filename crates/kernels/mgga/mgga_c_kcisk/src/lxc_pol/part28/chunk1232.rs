//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1232/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1232<F: Float>(t34107: F, t9945: F, t1799: F, t5185: F, t7718: F, t9679: F, t33003: F, t8851: F, t7261: F) -> (F, F, F, F, F, F, F) {
    let t35100 = t34107 * t9945;
    let t35101 = t1799 * t35100;
    let t35103 = t5185 * t7718;
    let t35104 = t9679 * t35103;
    let t35105 = t1799 * t35104;
    let t35107 = t33003 * t8851;
    let t35108 = t7261 * t35107;
    (t35100, t35101, t35103, t35104, t35105, t35107, t35108)
}
