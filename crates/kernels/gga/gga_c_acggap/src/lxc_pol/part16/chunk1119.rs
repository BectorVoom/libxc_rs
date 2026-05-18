//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1119/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1119<F: Float>(t1844: F, t322: F, t1165: F, t604: F, t7346: F, t1181: F, t2068: F, t39164: F, t2016: F, t9630: F, t1327: F, t507: F, t8888: F) -> (F, F, F, F, F) {
    let t39499 = t1844 * t322;
    let t39502 = t7346 * t1165 * t604 * t39499;
    let t39506 = t2068 * t1181 * t604 * t39164;
    let t39508 = t2016 * t9630;
    let t39511 = t8888 * t507 * t1327;
    (t39499, t39502, t39506, t39508, t39511)
}
