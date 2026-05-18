//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 622/1445 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk622<F: Float>(t1023: F, t3670: F, t128: F, t1457: F, t1033: F, t169: F, t3157: F, t1044: F) -> (F, F, F, F, F) {
    let t3671 = t3670 * t1023;
    let t3673 = t1457 * t128;
    let t3674 = t3673 * t1033;
    let t3676 = t169 * t3674 * t3157;
    let t3678 = t128 * t1044;
    let t3679 = t3678 * M_PI;
    (t3671, t3673, t3674, t3676, t3679)
}
