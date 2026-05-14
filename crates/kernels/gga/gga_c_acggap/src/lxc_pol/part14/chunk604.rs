//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 604/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk604<F: Float>(t1795: F, t360: F, t1083: F, t398: F, t1165: F, t1175: F, t5852: F, t1181: F, t1182: F, t1188: F, t5922: F, t1411: F, t1532: F, t1750: F, t3379: F, t1748: F, t4289: F) -> (F, F, F, F, F, F, F, F) {
    let t6226 = t1795 * t360;
    let t6228 = t398 * t1083 * t6226;
    let t6237 = t1165 * t5852 * t1175;
    let t6241 = t1181 * t5852 * t1182;
    let t6245 = t1165 * t5922 * t1188;
    let t6249 = t1165 * t1532 * t1411;
    let t6252 = t3379 * t1750;
    let t6255 = t1165 * t4289 * t1748;
    (t6226, t6228, t6237, t6241, t6245, t6249, t6252, t6255)
}
