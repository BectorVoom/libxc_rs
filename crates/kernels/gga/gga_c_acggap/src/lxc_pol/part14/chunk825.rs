//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 825/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk825<F: Float>(t7310: F, t7389: F, t7753: F, t7799: F, t435: F, t7322: F, t7323: F, t1072: F, t372: F, t721: F, t2019: F, t2059: F, t128: F, t576: F, t7475: F, t1108: F, t7736: F) -> (F, F, F, F, F, F, F) {
    let t31126 = t7310 * t7389;
    let t31128 = t7799 * t7753;
    let t31137 = t7322 * t7323 * t435;
    let t31140 = t31137 * t1072 * t372 * t721;
    let t31142 = t2019 * t2059;
    let t31146 = t576 * t7475 * t128;
    let t31160 = t7736 * t1108;
    (t31126, t31128, t31137, t31140, t31142, t31146, t31160)
}
