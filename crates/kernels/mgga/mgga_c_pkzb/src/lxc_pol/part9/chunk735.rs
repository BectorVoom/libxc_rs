//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 735/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk735<F: Float>(t4996: F, t5005: F, t5011: F, t5019: F, t5022: F, t5171: F, t5176: F, t5178: F, t5180: F, t5186: F, t5188: F, t5190: F, t123: F, t475: F, t574: F, t550: F) -> (F, F, F) {
    let t5320 = t4996 + t5005 - t5011 + t5171 + t5019 - t5022 + t5176 + t5178 + t5180 + t5186 - t5188 + t5190;
    let t5322 = t475 * t574 * t123;
    let t5324 = 0.56968947174242584612e-3 * t550 * t5322;
    (t5320, t5322, t5324)
}
