//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 983/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk983<F: Float>(t1882: F, t7111: F, t7038: F, t28496: F, t2862: F, t319: F, t7021: F, t875: F, t840: F, t871: F, t2749: F, t7045: F, t1248: F, t6260: F, t1476: F, t4299: F) -> (F, F, F, F, F, F, F, F, F) {
    let t29285 = t1882 * t7111;
    let t29287 = t1882 * t7038;
    let t29290 = t2862 * t319 * t28496;
    let t29293 = t7021 * t875;
    let t29295 = t840 * t871 * t29293;
    let t29299 = t840 * t2749 * t7045;
    let t29302 = t6260 * t1248;
    let t29304 = t840 * t871 * t29302;
    let t29307 = t1476 * t4299;
    (t29285, t29287, t29290, t29293, t29295, t29299, t29302, t29304, t29307)
}
