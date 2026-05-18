//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1027/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1027<F: Float>(t146: F, t147: F, t6567: F, t155: F, t158: F, t6165: F, t2004: F, t2123: F, t115: F, t658: F, t5: F, t2219: F) -> (F, F, F, F, F) {
    let t23163 = t146 * t147 * t6567;
    let t23171 = t155 * t158 * t6165;
    let t23219 = t2123 * t2004;
    let t23269 = t658 * t115;
    let t23270 = t23269 * t5;
    let t23315 = t2219 * t2219;
    (t23163, t23171, t23219, t23270, t23315)
}
