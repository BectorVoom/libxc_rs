//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1007/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1007<F: Float>(t8392: F, t9146: F, t599: F, t7943: F, t89: F, t1882: F, t9412: F, t9318: F, t9313: F, t9306: F, t161: F, t38061: F) -> (F, F, F, F, F, F, F) {
    let t41047 = t8392 * t9146;
    let t41050 = t89 * t7943 * t599;
    let t41064 = t1882 * t9412;
    let t41074 = t1882 * t9318;
    let t41076 = t1882 * t9313;
    let t41084 = t1882 * t9306;
    let t41093 = F::new(280.0) / F::new(243.0) * t89 * t38061 * t161;
    (t41047, t41050, t41064, t41074, t41076, t41084, t41093)
}
