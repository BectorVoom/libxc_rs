//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 678/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk678<F: Float>(t1243: F, t2890: F, t1552: F, t978: F, t1251: F, t2863: F, t542: F, t974: F, t496: F, t1576: F, t981: F, t1563: F, t9: F) -> (F, F, F, F, F, F, F, F) {
    let t8149 = t2890 * t1243;
    let t8159 = t1552 * t978;
    let t8160 = t8159 * t1251;
    let t8197 = t2863 * t1243;
    let t8199 = t542 * t974;
    let t8200 = t496 * t8199;
    let t8209 = t981 * t1576;
    let t8231 = t9 * t1563;
    (t8149, t8159, t8160, t8197, t8199, t8200, t8209, t8231)
}
