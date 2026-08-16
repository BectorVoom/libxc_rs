//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 562/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk562<F: Float>(t1887: F, t4684: F, t706: F, t1689: F, t4636: F, t4638: F, t4642: F, t4646: F, t4650: F, t587: F, t1640: F, t1644: F) -> (F, F, F, F, F, F) {
    let t4685 = t1887 * t4684;
    let t4686 = t706 * t4685;
    let t4689 = t1689 * t1689;
    let t4691 = F::cast_from(0.23744444444444444444e-1_f64) * t4636;
    let t4696 = t4691 + F::cast_from(0.11872222222222222222e-1_f64) * t4638 - F::cast_from(0.11872222222222222222e-1_f64) * t4642 + F::cast_from(0.35616666666666666666e-1_f64) * t4646 - F::cast_from(0.17808333333333333333e-1_f64) * t4650;
    let t4698 = F::cast_from(0.62182e-1_f64) * t4696 * t587;
    let t4699 = t1640 * t1644;
    (t4685, t4686, t4689, t4696, t4698, t4699)
}
