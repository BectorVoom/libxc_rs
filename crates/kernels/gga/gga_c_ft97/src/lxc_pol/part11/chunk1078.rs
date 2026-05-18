//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1078/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1078<F: Float>(t10082: F, t8392: F, t713: F, t9571: F, t505: F, t8608: F, t1934: F, t2349: F) -> (F, F, F, F) {
    let t42392 = t8392 * t10082;
    let t42394 = t9571 * t713;
    let t42399 = t8608 * t505;
    let t42404 = t1934 * t2349;
    (t42392, t42394, t42399, t42404)
}
