//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1017/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1017<F: Float>(t78693: F, t78725: F, t358: F, t497: F, t1326: F, t8417: F, t1851: F, t5704: F, t22883: F, t378: F, t23249: F, t47667: F, t487: F, t5617: F, t1328: F, t7943: F, t89: F) -> (F, F, F, F, F, F, F, F) {
    let t78726 = t78693 + t78725;
    let t91480 = t497 * t358;
    let t91493 = t1326 * t8417;
    let t91496 = t5704 * t1851;
    let t91504 = t378 * t22883;
    let t91539 = t47667 * t23249;
    let t91583 = t487 * t5617;
    let t91625 = 28.0 / 81.0 * t89 * t7943 * t1328;
    (t78726, t91480, t91493, t91496, t91504, t91539, t91583, t91625)
}
