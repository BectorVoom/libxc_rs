//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 423/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk423<F: Float>(t108: F, t692: F, t110: F, t146: F, t2022: F, t5: F, t2024: F, t675: F, t622: F, t671: F) -> (F, F, F, F, F) {
    let t2111 = t692 * t108;
    let t2113 = t146 * t2111 * t110;
    let t2114 = t5 * t2022;
    let t2115 = t2114 * t2024;
    let t2116 = t675 * t2115;
    let t2120 = t146 * t671 * t622;
    (t2111, t2113, t2114, t2116, t2120)
}
