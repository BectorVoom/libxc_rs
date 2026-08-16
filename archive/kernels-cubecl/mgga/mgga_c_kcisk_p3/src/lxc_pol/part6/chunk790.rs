//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 790/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk790<F: Float>(t12261: F, t2327: F, t535: F, t2097: F, t3722: F, t2285: F, t4460: F, t4435: F, t3696: F, t2318: F, t4416: F, t2306: F, t4346: F) -> (F, F, F, F, F, F, F) {
    let t21674 = t12261 * t2327;
    let t21675 = t535 * t21674;
    let t21748 = t2097 * t3722;
    let t21764 = t2285 * t4460;
    let t21869 = t2285 * t4435;
    let t21872 = t2097 * t3696;
    let t21902 = t2318 * t4416;
    let t21969 = t2306 * t4346;
    (t21675, t21748, t21764, t21869, t21872, t21902, t21969)
}
