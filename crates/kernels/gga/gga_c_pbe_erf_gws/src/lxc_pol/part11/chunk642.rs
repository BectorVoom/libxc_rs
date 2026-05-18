//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 642/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk642<F: Float>(t147: F, t6045: F, t551: F, t1480: F, t1473: F, t759: F, t922: F, t1378: F, t285: F, t799: F, t1497: F, t751: F) -> (F, F, F, F, F, F, F, F, F) {
    let t6046 = t6045 * t147;
    let t6047 = t6046 * t551;
    let t6049 = F::new(0.16396719238543588599e-3) * t6047 * t1480;
    let t6053 = F::new(0.15965645347006145458e0) * t1473 * t759;
    let t6054 = t922 * t147;
    let t6055 = t6054 * t1378;
    let t6056 = t799 * t285;
    let t6058 = F::new(0.45692190944741466895e-5) * t6055 * t6056;
    let t6064 = F::new(0.59871170051273045469e-1) * t751 * t1497;
    (t6046, t6047, t6049, t6053, t6054, t6055, t6056, t6058, t6064)
}
