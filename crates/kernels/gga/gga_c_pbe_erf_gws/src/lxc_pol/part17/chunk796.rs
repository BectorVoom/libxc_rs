//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 796/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk796<F: Float>(t147: F, t922: F, t1378: F, t285: F, t799: F, t1488: F, t751: F, t1492: F, t1497: F, t309: F, t310: F, t311: F) -> (F, F, F, F, F) {
    let t6054 = t922 * t147;
    let t6055 = t6054 * t1378;
    let t6056 = t799 * t285;
    let t6058 = F::new(0.45692190944741466895e-5) * t6055 * t6056;
    let t6059 = t751 * t1488;
    let t6061 = t751 * t1492;
    let t6064 = F::new(0.59871170051273045469e-1) * t751 * t1497;
    let t6072 = F::new(1.0) / t311 / t310 / t309;
    (t6058, t6059, t6061, t6064, t6072)
}
