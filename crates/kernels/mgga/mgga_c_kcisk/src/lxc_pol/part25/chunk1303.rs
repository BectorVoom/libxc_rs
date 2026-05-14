//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1303/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1303<F: Float>(t1869: F, t32903: F, t34089: F, t2442: F, t4803: F, t9679: F, t33044: F, t34107: F, t5054: F, t1799: F, t33040: F, t34093: F, t32995: F, t34073: F, t32909: F, t34154: F) -> (F, F, F, F, F, F, F, F) {
    let t116683 = t1869 * t32903 * t34089;
    let t116687 = t1869 * t9679 * t2442 * t4803;
    let t116690 = t5054 * t34107 * t33044;
    let t116695 = t1799 * t34093 * t33040;
    let t116698 = t5054 * t34093 * t33044;
    let t116701 = 0.69444444444444444446e-2 * t34073 * t32995;
    let t116703 = 0.69444444444444444446e-2 * t34073 * t32909;
    let t116705 = 0.26805555555555555556e-2 * t34154 * t32909;
    (t116683, t116687, t116690, t116695, t116698, t116701, t116703, t116705)
}
