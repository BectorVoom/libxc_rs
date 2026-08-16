//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2409/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2409<F: Float>(t10995: F, t123: F, t2434: F, t2771: F, t10504: F, t138: F, t2438: F, t2828: F, t11044: F, t11050: F, t11015: F, t2461: F) -> (F, F, F, F) {
    let t41052 = t10995 * t123 * t2434 * t2771;
    let t41056 = t10504 * t138 * t2438 * t2828;
    let t41058 = t11044 * t11050;
    let t41060 = t2461 * t11015;
    (t41052, t41056, t41058, t41060)
}
