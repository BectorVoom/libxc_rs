//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1159/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1159<F: Float>(t1181: F, t26956: F, t7564: F, t8600: F, t7433: F, t9601: F, t1165: F, t26757: F, t604: F, t7413: F, t9583: F, t2068: F, t25706: F) -> (F, F, F, F, F) {
    let t40003 = t7564 * t1181 * t8600 * t26956;
    let t40005 = t7433 * t9601;
    let t40009 = t7413 * t1165 * t604 * t26757;
    let t40011 = t7433 * t9583;
    let t40015 = t2068 * t1181 * t604 * t25706;
    (t40003, t40005, t40009, t40011, t40015)
}
