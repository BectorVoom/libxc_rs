//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 839/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk839<F: Float>(t5135: F, t66: F, t168: F, t167: F, t180: F, t173: F, t1765: F) -> (F, F, F, F) {
    let t5285 = 1.0 / t66 / t5135;
    let t5286 = t168 * t5285;
    let t5289 = 0.37792653007779990369e-1 * t167 * t5286 * t180;
    let t5295 = t1765 * t173;
    let t5296 = t167 * t5295;
    (t5286, t5289, t5295, t5296)
}
