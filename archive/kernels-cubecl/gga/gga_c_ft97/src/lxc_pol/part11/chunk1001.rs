//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1001/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1001<F: Float>(t157: F, t40465: F, t8392: F, t9425: F, t24: F, t32905: F, t2159: F, t8232: F, t9094: F, t9129: F, t1882: F, t9109: F) -> (F, F, F, F, F, F, F) {
    let t40808 = t40465 * t157;
    let t40828 = t8392 * t9425;
    let t40830 = t24 * t32905;
    let t40835 = t8232 * t2159;
    let t40837 = t8392 * t9094;
    let t40840 = t8392 * t9129;
    let t40847 = t1882 * t9109;
    (t40808, t40828, t40830, t40835, t40837, t40840, t40847)
}
