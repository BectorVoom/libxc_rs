//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1065/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1065<F: Float>(t1434: F, t27837: F, t681: F, t193: F, t27742: F, t6109: F, t743: F, t747: F, t6119: F, t9802: F, t505: F, t3690: F, t96934: F, t1900: F, t6: F, t91: F, t9890: F) -> (F, F, F, F, F, F) {
    let t108171 = t1434 * t681 * t27837;
    let t108172 = 2.0 / 3.0 * t108171;
    let t108176 = t6109 * t193 * t743 * t27742 * t747;
    let t108178 = t9802 * t6119;
    let t108179 = t505 * t747;
    let t108182 = t96934 * t108178 * t3690 * t108179;
    let t108186 = t91 * t9890 * t6 * t1900;
    (t108171, t108172, t108176, t108179, t108182, t108186)
}
