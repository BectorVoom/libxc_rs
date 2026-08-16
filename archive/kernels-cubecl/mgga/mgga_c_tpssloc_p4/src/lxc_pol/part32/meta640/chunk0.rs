//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2058/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2058<F: Float>(t28: F, t40772: F, t1649: F, t2752: F, t1437: F, t6509: F, t1864: F, t4021: F, t1410: F, t9231: F, t2240: F, t3961: F) -> (F, F, F, F, F, F) {
    let t89953 = t40772 * t28;
    let t89992 = t2752 * t1649;
    let t90090 = t6509 * t1437;
    let t90094 = t1864 * t4021;
    let t90098 = t9231 * t1410;
    let t90101 = t2240 * t3961;
    (t89953, t89992, t90090, t90094, t90098, t90101)
}
