//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 655/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk655<F: Float>(t2213: F, t9099: F, t2230: F, t558: F, t574: F, t1882: F, t2159: F, t1647: F, t569: F, t616: F, t2218: F, t1554: F, t525: F) -> (F, F, F, F, F, F) {
    let t9100 = t9099 * t2213;
    let t9104 = t574 * t2230 * t558;
    let t9106 = t1882 * t2159;
    let t9109 = t569 * t616 * t1647;
    let t9112 = t1882 * t2218;
    let t9114 = t1554 * t525;
    (t9100, t9104, t9106, t9109, t9112, t9114)
}
