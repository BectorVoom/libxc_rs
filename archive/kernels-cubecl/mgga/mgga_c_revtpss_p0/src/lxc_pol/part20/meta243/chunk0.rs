//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1052/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1052<F: Float>(t1096: F, t3270: F, t11121: F, t1071: F, t3046: F, t268: F, t271: F, t7021: F) -> (F, F, F, F) {
    let t11122 = t3270 * t1096;
    let t11123 = t11121 * t11122;
    let t11128 = t3046 * t1071;
    let t11132 = t268 * t7021 * t271;
    (t11122, t11123, t11128, t11132)
}
