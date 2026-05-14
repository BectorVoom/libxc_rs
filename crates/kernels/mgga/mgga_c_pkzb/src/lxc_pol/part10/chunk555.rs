//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 555/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk555<F: Float>(t2203: F, t2204: F, t2172: F, t2175: F, t2187: F) -> (F, F, F) {
    let t2205 = t2203 * t2204;
    let t2207 = 4.0 / 9.0 * t2172;
    let t2209 = t2207 - 2.0 / 3.0 * t2175 + t2187;
    (t2205, t2207, t2209)
}
