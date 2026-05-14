//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1186/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1186<F: Float>(t11938: F, t11944: F, t11954: F, t1670: F, t1674: F, t1675: F, t1713: F, t20040: F, t20043: F, t20046: F, t20048: F, t20049: F, t20052: F, t2853: F, t4099: F, t4822: F, t96: F) -> (F,) {
    let t24654 = 12.0 * t1670 * t1674 * t4822 + 12.0 * t1674 * t1675 * t4099 + 6.0 * t1713 * t2853 * t96 - t11938 - t11944 - t11954 - t20040 - t20043 - t20046 - t20048 - t20049 + t20052;
    (t24654,)
}
