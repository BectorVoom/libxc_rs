//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 1166/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk1166<F: Float>(t144: F, t3095: F, t3094: F, t3954: F, t128: F, t3141: F, t33655: F, t5462: F, t623: F, t11320: F, t11322: F, t1932: F) -> (F, F, F, F) {
    let t34447 = t3095 * t144;
    let t34449 = t3094 * t34447 * t3954;
    let t34454 = t5462 * t33655 * t3141 * t623 * t128;
    let t34457 = t1932 * t11320 * t11322;
    (t34447, t34449, t34454, t34457)
}
