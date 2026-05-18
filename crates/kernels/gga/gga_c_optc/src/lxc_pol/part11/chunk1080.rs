//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1080/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1080<F: Float>(t4694: F, t7030: F, t4703: F, t7073: F, t4699: F, t4637: F, t658: F, t13061: F, t1998: F, t2042: F, t4580: F, t1994: F) -> (F, F, F, F, F, F, F) {
    let t38148 = t7030 * t4694;
    let t38172 = t7073 * t4703;
    let t38174 = t7073 * t4699;
    let t38298 = t4637 * t658;
    let t38318 = t13061 * t1998;
    let t38332 = t2042 * t4580;
    let t38339 = t13061 * t1994;
    (t38148, t38172, t38174, t38298, t38318, t38332, t38339)
}
