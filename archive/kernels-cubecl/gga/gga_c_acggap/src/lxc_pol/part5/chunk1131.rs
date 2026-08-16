//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1131/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1131<F: Float>(t1008: F, t6110: F, t1005: F, t5971: F, t1089: F, t175: F, t322: F, t384: F, t5506: F, t1734: F, t879: F, t5826: F) -> (F, F, F, F, F) {
    let t20238 = t1008 * t6110;
    let t20263 = t1005 * t5971;
    let t20268 = t384 * t1089 * t175 * t5506 * t322;
    let t20273 = t384 * t1089 * t175 * t1734 * t879;
    let t20275 = t1005 * t5826;
    (t20238, t20263, t20268, t20273, t20275)
}
