//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 951/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk951<F: Float>(t1165: F, t3361: F, t3809: F, t540: F, t3346: F, t14047: F, t4908: F, t4680: F, t4907: F, t1140: F, t4773: F, t4430: F, t3375: F, t4959: F, t1163: F, t4958: F) -> (F, F, F, F, F, F, F, F) {
    let t18475 = t3361 * t1165 * t540 * t3809;
    let t18480 = t3361 * t1165 * t540 * t3346;
    let t18482 = t14047 * t4908;
    let t18485 = t3361 * t4680 * t4907;
    let t18487 = t1140 * t4773;
    let t18489 = t1140 * t4430;
    let t18499 = t3375 * t4959;
    let t18502 = t1163 * t4680 * t4958;
    (t18475, t18480, t18482, t18485, t18487, t18489, t18499, t18502)
}
