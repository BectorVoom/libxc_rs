//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 568/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk568<F: Float>(t466: F, t8282: F, t1775: F, t1797: F, t1783: F, t1802: F, t458: F, t2: F, t8216: F, t7825: F, t1787: F, t7829: F, t1806: F, t462: F, t8263: F, t8267: F, t8272: F, t8278: F, t92: F) -> (F, F, F, F) {
    let t8283 = t8282 * t466;
    let t8285 = t1775 * t1797;
    let t8287 = t1775 * t1783;
    let t8289 = t458 * t1802;
    let t8291 = t8216 * t2;
    let t8292 = t8291 * t7825;
    let t8295 = t1787 * t7829;
    let t8298 = t458 * t1806;
    let t8299 = 6.0 * t462 * t8263 - t462 * t8267 / 3.0 - 6.0 * t92 * t8272 - 10.0 / 27.0 * t462 * t8278 - 4.0 / 9.0 * t8283 + t8285 / 3.0 + 2.0 / 9.0 * t8287 - 2.0 * t8289 - 2.0 * t462 * t8292 - 2.0 * t462 * t8295 + t8298;
    (t8291, t8292, t8295, t8299)
}
