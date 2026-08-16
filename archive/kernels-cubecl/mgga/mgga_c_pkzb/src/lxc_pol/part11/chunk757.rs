//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 757/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk757<F: Float>(t6087: F, t2463: F, t418: F, t2411: F, t300: F, t1478: F, t154: F, t386: F, t385: F, t465: F, t931: F, t179: F, t824: F) -> (F, F, F, F, F, F, F) {
    let t6348 = F::cast_from(0.53272592592592592592e-1_f64) * t6087;
    let t6362 = F::cast_from(1.0_f64) / t2463 / t418;
    let t6366 = t300 * t2411;
    let t6377 = t154 * t1478 * t386;
    let t6379 = F::cast_from(5.0_f64) / F::cast_from(1296.0_f64) * t385 * t6377;
    let t6380 = t465 * t931;
    let t6382 = t179 * t6380 * t824;
    (t6348, t6362, t6366, t6377, t6379, t6380, t6382)
}
