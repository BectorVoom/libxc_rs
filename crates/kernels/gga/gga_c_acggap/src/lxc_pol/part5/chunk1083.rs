//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1083/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1083<F: Float>(t14852: F, t14854: F, t14856: F, t11591: F, t11597: F, t11566: F, t11570: F, t11574: F, t11578: F, t11582: F, t11586: F, t11596: F) -> (F, F, F, F, F, F) {
    let t19396 = F::new(0.11393789434848516923e-2) * t14852;
    let t19397 = F::new(0.10389515463408878255e3) * t14854;
    let t19398 = F::new(0.70178683471615754484e1) * t14856;
    let t19399 = F::new(0.11393789434848516922e-2) * t11591;
    let t19400 = F::new(0.10389515463408878255e3) * t11597;
    let t19401 = -t19396 - t19397 + t19398 + t11566 + t11570 - t11574 + t11578 - t11582 - t11586 - t19399 + t11596 - t19400;
    (t19396, t19397, t19398, t19399, t19400, t19401)
}
