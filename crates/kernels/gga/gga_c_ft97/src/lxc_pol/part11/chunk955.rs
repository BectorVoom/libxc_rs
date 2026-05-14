//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 955/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk955<F: Float>(t2610: F, t38953: F, t762: F, t9802: F, t10076: F, t8392: F, t754: F, t9895: F, t2542: F, t737: F, t10082: F, t713: F, t9571: F, t505: F, t8608: F, t1934: F, t2349: F) -> (F, F, F, F, F, F, F, F, F) {
    let t42360 = t38953 * t2610;
    let t42362 = t9802 * t762;
    let t42374 = t8392 * t10076;
    let t42376 = t9895 * t754;
    let t42385 = t737 * t2542;
    let t42392 = t8392 * t10082;
    let t42394 = t9571 * t713;
    let t42399 = t8608 * t505;
    let t42404 = t1934 * t2349;
    (t42360, t42362, t42374, t42376, t42385, t42392, t42394, t42399, t42404)
}
