//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 834/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk834<F: Float>(t13917: F, t8318: F, t1580: F, t6459: F, t6473: F, t1610: F, t8432: F, t1149: F, t7724: F, t2527: F, t7715: F, t6666: F) -> (F, F, F, F, F, F) {
    let t27920 = t13917 * t8318;
    let t27921 = t1580 * t27920;
    let t27925 = t6459 * t6473;
    let t28036 = t8432 * t1610;
    let t28152 = t7724 * t1149;
    let t28208 = t7715 * t2527;
    let t28209 = t6666 * t28208;
    (t27921, t27925, t28036, t28152, t28208, t28209)
}
