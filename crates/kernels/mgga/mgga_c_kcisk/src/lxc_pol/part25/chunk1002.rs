//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1002/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1002<F: Float>(t15930: F, t6675: F, t5192: F, t5182: F, t140: F, t3737: F, t6672: F, t6677: F, t11197: F, t11663: F, t1693: F, t16997: F, t17005: F, t17135: F, t17139: F, t17143: F, t17150: F, t17154: F, t17159: F, t17734: F, t17740: F, t17744: F, t17748: F, t17751: F, t17755: F, t17757: F, t4823: F, t4827: F, t7278: F) -> (F, F, F, F) {
    let t17759 = t6675 * t15930;
    let t17760 = t5192 * t17759;
    let t17761 = t5182 * t17760;
    let t17764 = t140 * t3737 * t6672;
    let t17765 = t17764 * t6677;
    let t17766 = 0.3684876543209876543e-2 * t17765;
    let t17767 = 0.73697530864197530862e-3 * t17135 - 0.13265555555555555555e-1 * t17139 + 0.18424382716049382715e-2 * t17143 + 0.74498e-1 * t4823 * t16997 - 0.43134342e-1 * t11197 * t17005 - 0.33163888888888888888e-2 * t11663 - 0.33163888888888888888e-2 * t17150 - 0.33163888888888888888e-2 * t17154 - 0.16581944444444444444e-2 * t17159 - 0.193e0 * t1693 * t17734 + 0.193e0 * t7278 * t4827 + t17740 - 0.3684876543209876543e-3 * t17744 - 0.22109259259259259258e-2 * t17748 - t17751 + 0.33163888888888888888e-2 * t17755 - 0.44218518518518518517e-2 * t17757 + 0.99491666666666666664e-2 * t17761 + t17766;
    (t17759, t17761, t17765, t17767)
}
