//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1270/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1270<F: Float>(t32359: F, t9532: F, t109515: F, t9536: F, t123: F, t2734: F, t32357: F, t32338: F, t9515: F, t32345: F, t3973: F, t32388: F, t9529: F, t18681: F, t2737: F, t2739: F) -> (F, F, F, F, F, F, F) {
    let t109729 = t32359 * t9532;
    let t109738 = t9536 * t109515;
    let t109749 = t2734 * t32357 * t123;
    let t109756 = t9515 * t32338;
    let t109760 = t9536 * t3973 * t32345;
    let t109793 = t9529 * t32388;
    let t109797 = 0.19290123456790123457e-2 * t2737 * t18681 * t2739;
    (t109729, t109738, t109749, t109756, t109760, t109793, t109797)
}
