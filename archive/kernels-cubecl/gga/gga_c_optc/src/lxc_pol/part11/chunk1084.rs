//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1084/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1084<F: Float>(t1256: F, t745: F, t193: F, t4752: F, t6654: F, t2204: F, t4611: F, t5068: F, t7274: F, t999: F, t5053: F, t8393: F) -> (F, F, F, F, F) {
    let t39009 = t745 * t1256;
    let t39030 = t193 * t6654 * t4752;
    let t39066 = t4611 * t2204;
    let t39204 = t999 * t7274 * t5068;
    let t39288 = t5053 * t8393;
    (t39009, t39030, t39066, t39204, t39288)
}
