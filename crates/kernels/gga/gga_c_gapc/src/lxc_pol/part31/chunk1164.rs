//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1164/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1164<F: Float>(t15507: F, t8: F, t29867: F, t332: F, t6: F, t7875: F, t1084: F, t291: F, t4052: F, t3095: F, t6182: F, t9438: F) -> (F, F, F, F) {
    let t33521 = F::new(1.0) / t8 / t15507;
    let t33527 = t7875 * t332 * t6 * t29867;
    let t33528 = t1084 * t4052 * t33521 * t291 * t33527;
    let t33530 = t3095 * t291;
    let t33532 = t9438 * t33530 * t6182;
    (t33521, t33528, t33530, t33532)
}
