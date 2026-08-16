//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 511/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk511<F: Float>(t2089: F, t827: F, t22: F, t3118: F, t2097: F, t45: F, t2105: F, t3696: F, t2059: F, t443: F, t3859: F, t212: F, t23: F) -> (F, F, F, F, F, F, F) {
    let t5736 = t827 * t2089;
    let t5744 = t22 * t3118;
    let t5765 = t45 * t2097;
    let t5770 = t3696 * t2105;
    let t5802 = t443 * t2059;
    let t5804 = t3859 * t2059;
    let t5814 = F::cast_from(1.0_f64) / t23 / t212;
    (t5736, t5744, t5765, t5770, t5802, t5804, t5814)
}
