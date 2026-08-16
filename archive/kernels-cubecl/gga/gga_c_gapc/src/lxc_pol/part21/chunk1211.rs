//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 1211/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk1211<F: Float>(t28065: F, t3709: F, t11388: F, t8636: F, t11380: F, t1448: F, t8788: F, t11509: F, t5626: F, t11513: F, t5392: F, t5395: F) -> (F, F, F, F, F) {
    let t34982 = t3709 * t28065;
    let t34984 = t11388 * t8636;
    let t34987 = t11380 * t1448 * t8788;
    let t34989 = t11509 * t5626;
    let t34992 = t5395 * t11513 * t5392;
    (t34982, t34984, t34987, t34989, t34992)
}
