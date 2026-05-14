//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1117/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1117<F: Float>(t108157: F, t27851: F, t6109: F, t681: F, t1434: F, t27837: F, t6119: F, t9802: F, t1900: F, t6: F, t91: F, t9890: F, t2492: F, t1154: F, t668: F, t24543: F, t27768: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t108158 = t108157 / 6.0;
    let t108160 = t6109 * t681 * t27851;
    let t108161 = t108160 / 6.0;
    let t108171 = t1434 * t681 * t27837;
    let t108172 = 2.0 / 3.0 * t108171;
    let t108178 = t9802 * t6119;
    let t108186 = t91 * t9890 * t6 * t1900;
    let t108187 = t2492 * t6119;
    let t108188 = t1154 * t668;
    let t108210 = t24543 * t27768;
    (t108158, t108160, t108161, t108171, t108172, t108178, t108186, t108187, t108188, t108210)
}
