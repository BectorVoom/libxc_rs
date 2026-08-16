//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 1267/1426 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk1267<F: Float>(t19639: F, t34317: F, t1030: F, t3008: F, t33158: F, t34447: F, t3949: F, t9203: F, t128: F, t3141: F, t33655: F, t5541: F, t583: F) -> (F, F, F, F) {
    let t35121 = t34317 * t19639;
    let t35124 = t1030 * t33158 * t3008;
    let t35127 = t9203 * t34447 * t3949;
    let t35132 = t5541 * t33655 * t3141 * t583 * t128;
    (t35121, t35124, t35127, t35132)
}
