//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2527/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2527<F: Float>(t2770: F, t340: F, t2403: F, t4389: F, t4386: F, t13543: F, t699: F, t13547: F, t13556: F, t13529: F, t13533: F, t344: F, t42308: F, t60: F) -> (F, F, F, F, F, F, F, F, F) {
    let t48143 = t340 * t2770;
    let t48155 = t2403 * t4389;
    let t48157 = t2403 * t4386;
    let t48159 = t699 * t13543;
    let t48161 = t699 * t13547;
    let t48163 = t699 * t13556;
    let t48165 = t699 * t13529;
    let t48167 = t699 * t13533;
    let t48180 = t60 * t42308 * t344;
    (t48143, t48155, t48157, t48159, t48161, t48163, t48165, t48167, t48180)
}
