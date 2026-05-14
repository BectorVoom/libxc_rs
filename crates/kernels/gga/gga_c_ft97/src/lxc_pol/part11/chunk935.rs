//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 935/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk935<F: Float>(t1882: F, t9762: F, t2371: F, t2404: F, t2373: F, t2405: F, t446: F, t713: F, t9578: F, t9744: F, t193: F, t89: F, t9692: F, t2345: F, t2348: F, t41468: F) -> (F, F, F, F, F, F, F) {
    let t41877 = t1882 * t9762;
    let t41879 = t2404 * t2371;
    let t41880 = t2405 * t2373;
    let t41882 = t446 * t41879 * t41880;
    let t41884 = t9578 * t713;
    let t41886 = t446 * t9744 * t41884;
    let t41891 = t89 * t193 * t2371 * t9692 * t713;
    let t41895 = t89 * t2345 * t2348 * t41468;
    (t41877, t41880, t41882, t41884, t41886, t41891, t41895)
}
