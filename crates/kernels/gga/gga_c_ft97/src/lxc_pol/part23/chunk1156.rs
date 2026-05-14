//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1156/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1156<F: Float>(t6249: F, t7012: F, t96535: F, t24330: F, t28587: F, t108639: F, t7009: F, t4125: F, t703: F, t2691: F, t98519: F, t6242: F, t6999: F, t108685: F, t6243: F, t6250: F) -> (F, F, F, F, F, F, F, F) {
    let t111895 = t6249 * t96535 * t7012;
    let t111901 = t6249 * t24330 * t28587;
    let t111908 = t7009 * t108639;
    let t111910 = t703 * t4125;
    let t111915 = t2691 * t98519;
    let t111935 = t6242 * t96535 * t6999;
    let t111953 = 0.17780800291358024692e0 * t6242 * t108685 * t6243;
    let t111956 = 0.17780800291358024692e0 * t6249 * t108685 * t6250;
    (t111895, t111901, t111908, t111910, t111915, t111935, t111953, t111956)
}
