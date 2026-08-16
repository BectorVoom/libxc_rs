//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 644/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk644<F: Float>(t1753: F, t301: F, t1532: F, t1181: F, t1782: F, t3201: F, t336: F, t1143: F, t1713: F, t1788: F, t3621: F, t174: F, t1795: F) -> (F, F, F, F, F) {
    let t6269 = t1753 * t301;
    let t6270 = t1532 * t6269;
    let t6271 = t1181 * t6270;
    let t6279 = t336 * t3201 * t1782;
    let t6283 = t336 * t1143 * t1713;
    let t6286 = t3621 * t1788;
    let t6288 = t174 * t1795;
    (t6271, t6279, t6283, t6286, t6288)
}
