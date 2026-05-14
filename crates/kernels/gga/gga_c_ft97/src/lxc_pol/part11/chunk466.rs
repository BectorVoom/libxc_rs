//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 466/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk466<F: Float>(t2701: F, t274: F, t2380: F, t231: F, t123: F, t194: F, t805: F) -> (F, F, F, F) {
    let t2702 = t2701 * t274;
    let t2704 = t2380 * t274;
    let t2705 = t231 * t2704;
    let t2710 = t123 / t805 / t194;
    (t2702, t2704, t2705, t2710)
}
