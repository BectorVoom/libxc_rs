//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2891/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2891<F: Float>(t60079: F, t60158: F, t60185: F, t60214: F, t60242: F, t60279: F, t60300: F, t60329: F, t17191: F, t942: F, t2929: F, t5769: F) -> (F, F, F) {
    let t60332 = t60079 + t60158 + t60185 + t60214 + t60242 + t60279 + t60300 + t60329;
    let t60338 = t17191 * t942;
    let t60343 = t5769 * t2929;
    (t60332, t60338, t60343)
}
