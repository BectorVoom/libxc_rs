//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 774/1311 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk774<F: Float>(t3: F, t5: F, t8785: F, t8784: F, t8789: F, t3100: F, t664: F, t3044: F) -> (F, F, F, F, F, F) {
    let pi = F::cast_from(M_PI);
    let t9059 = t3 * t5;
    let t9060 = t9059 * t8785;
    let t9061 = t8784 * t9060;
    let t9062 = t9061 * t8789;
    let t9064 = t3100 * t664;
    let t9066 = t3044 * pi;
    (t9059, t9060, t9061, t9062, t9064, t9066)
}
