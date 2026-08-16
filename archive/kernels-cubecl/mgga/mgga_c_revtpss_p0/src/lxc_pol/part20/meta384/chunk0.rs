//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1402/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1402<F: Float>(t10504: F, t138: F, t2438: F, t2828: F, t11044: F, t11050: F, t11015: F, t2461: F, t11010: F, t689: F, t779: F, t2769: F, t786: F, t861: F) -> (F, F, F, F, F) {
    let t41056 = t10504 * t138 * t2438 * t2828;
    let t41058 = t11044 * t11050;
    let t41060 = t2461 * t11015;
    let t41063 = t689 * t779 * t11010;
    let t41066 = t786 * t861 * t2769;
    (t41056, t41058, t41060, t41063, t41066)
}
