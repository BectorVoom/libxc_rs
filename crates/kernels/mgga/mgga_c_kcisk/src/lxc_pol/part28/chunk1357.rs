//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1357/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1357<F: Float>(t34103: F, t34107: F, t6713: F, t1799: F, t32903: F, t35103: F, t22591: F, t5185: F, t9679: F, t117066: F, t9945: F, t1785: F, t36247: F, t7261: F, t8851: F, t2469: F, t33003: F, t7274: F) -> (F, F, F, F, F, F) {
    let t121219 = t6713 * t34107 * t34103;
    let t121222 = t1799 * t32903 * t35103;
    let t121226 = t1799 * t9679 * t5185 * t22591;
    let t121229 = t1799 * t117066 * t9945;
    let t121236 = t7261 * t36247 * t8851 * t1785;
    let t121241 = t7261 * t33003 * t2469 * t7274;
    (t121219, t121222, t121226, t121229, t121236, t121241)
}
