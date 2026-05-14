//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 885/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk885<F: Float>(t2312: F, t2320: F, t6087: F, t2463: F, t418: F, t2411: F, t300: F) -> (F, F, F, F) {
    let t6337 = t2312 * t2320;
    let t6348 = 0.53272592592592592592e-1 * t6087;
    let t6362 = 1.0 / t2463 / t418;
    let t6366 = t300 * t2411;
    (t6337, t6348, t6362, t6366)
}
