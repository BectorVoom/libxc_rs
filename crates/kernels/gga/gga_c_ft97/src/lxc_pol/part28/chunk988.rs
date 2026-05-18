//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 988/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk988<F: Float>(t32968: F, t376: F, t89: F, t32972: F, t1984: F, t32869: F, t32906: F, t72: F, t32988: F, t375: F, t23649: F, t32926: F) -> (F, F, F, F, F, F) {
    let t139413 = t89 * t376 * t32968;
    let t139416 = t89 * t376 * t32972;
    let t139418 = t1984 * t32869;
    let t139431 = t72 * t32906;
    let t139453 = t89 * t375 * t32988;
    let t139485 = t23649 * t32926;
    (t139413, t139416, t139418, t139431, t139453, t139485)
}
