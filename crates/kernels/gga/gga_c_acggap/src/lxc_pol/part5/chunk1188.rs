//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1188/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1188<F: Float>(t14712: F, t14717: F, t14718: F, t14719: F, t14720: F, t2959: F, t2961: F, t2963: F, t2966: F, t5397: F, t5401: F, t6590: F, t14725: F, t14726: F, t2971: F, t2984: F, t5026: F, t5032: F, t5036: F, t5405: F, t5409: F, t6020: F, t6594: F, t6598: F, t6601: F) -> (F, F) {
    let t24672 = -t14712 - 0.11696447245269292414e1 * t2959 - 0.10389515463408878255e3 * t2961 + 12.0 * t5397 + 0.14649157844805236043e-2 * t2963 - 0.36622894612013090108e-3 * t2966 + t14717 - t14718 - t14719 + t14720 + 24.0 * t6590 - 4.0 * t5401;
    let t24683 = 12.0 * t5405 + 24.0 * t5409 + 0.70178683471615754484e1 * t5026 + 6.0 * t6594 + 192.0 * t2971 - t14725 - t14726 - 0.35089341735807877242e1 * t2984 + 120.0 * t5032 - t6020 - 64.0 * t5036 + 4.0 * t6598 + 24.0 * t6601;
    (t24672, t24683)
}
