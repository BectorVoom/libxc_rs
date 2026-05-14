//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 491/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk491<F: Float>(t2097: F, t45: F, t2105: F, t3696: F, t2059: F, t443: F, t3859: F, t212: F, t23: F, t6: F, t161: F, t1048: F, t9: F, t7: F, t171: F, t156: F, t3122: F) -> (F, F, F, F, F, F, F, F, F) {
    let t5765 = t45 * t2097;
    let t5770 = t3696 * t2105;
    let t5802 = t443 * t2059;
    let t5804 = t3859 * t2059;
    let t5814 = 1.0 / t23 / t212;
    let t5815 = t6 * t5814;
    let t5816 = t161 * t5815;
    let t5821 = 1.0 / t9 / t1048;
    let t5822 = t7 * t5821;
    let t5823 = t171 * t5822;
    let t5827 = t156 * t3122;
    (t5765, t5770, t5802, t5804, t5814, t5816, t5821, t5823, t5827)
}
