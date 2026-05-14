//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 748/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk748<F: Float>(t22167: F, t788: F, t1091: F, t5330: F, t10703: F, t19517: F, t10492: F, t1248: F, t4917: F, t10485: F, t4139: F, t5225: F, t2862: F, t871: F, t1212: F, t5299: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t22168 = t788 * t22167;
    let t22177 = t5330 * t1091;
    let t22178 = t10703 * t22177;
    let t22182 = t19517 * t1091;
    let t22183 = t10492 * t22182;
    let t22186 = t4917 * t1248;
    let t22187 = t10485 * t22186;
    let t22188 = t4139 * t22187;
    let t22194 = t5225 * t1248;
    let t22196 = t2862 * t871 * t22194;
    let t22199 = t1212 * t5299;
    (t22168, t22177, t22178, t22182, t22183, t22186, t22187, t22188, t22194, t22196, t22199)
}
