//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 948/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk948<F: Float>(t13263: F, t1545: F, t3379: F, t4291: F, t4708: F, t1165: F, t3290: F, t3361: F, t6138: F, t1163: F, t1539: F, t15560: F, t3372: F, t4372: F, t3431: F, t4447: F) -> (F, F, F, F, F, F, F) {
    let t18301 = t13263 * t1545;
    let t18303 = t3379 * t4291;
    let t18305 = t3379 * t4708;
    let t18309 = t3361 * t1165 * t6138 * t3290;
    let t18321 = t1163 * t1165 * t15560 * t1539;
    let t18323 = t3372 * t4372;
    let t18329 = t3431 * t4447;
    (t18301, t18303, t18305, t18309, t18321, t18323, t18329)
}
