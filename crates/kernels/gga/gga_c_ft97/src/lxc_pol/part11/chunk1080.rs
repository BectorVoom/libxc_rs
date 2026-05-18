//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1080/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1080<F: Float>(t761: F, t9570: F, t766: F, t9571: F, t1882: F, t9989: F, t10059: F, t10004: F, t2576: F, t8232: F, t241: F, t41752: F) -> (F, F, F, F, F, F, F) {
    let t42416 = t761 * t9570;
    let t42417 = t9571 * t766;
    let t42422 = t1882 * t9989;
    let t42424 = t1882 * t10059;
    let t42430 = t1882 * t10004;
    let t42455 = t8232 * t2576;
    let t42469 = t41752 * t241;
    (t42416, t42417, t42422, t42424, t42430, t42455, t42469)
}
