//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1008/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1008<F: Float>(t12043: F, t12073: F, t12085: F, t12086: F, t12106: F, t12131: F, t12133: F, t12145: F, t502: F, t3751: F, t617: F, t1628: F, t3745: F) -> (F, F, F, F) {
    let t12148 = t12043 + t12073 + t12085 + t12086 + t12106 + t12131 + t12133 + t12145;
    let t12149 = t502 * t12148;
    let t12150 = t617 * t3751;
    let t12153 = t1628 * t3745;
    (t12148, t12149, t12150, t12153)
}
