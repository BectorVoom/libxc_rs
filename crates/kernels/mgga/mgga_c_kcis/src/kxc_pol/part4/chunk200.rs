//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 200/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk200<F: Float>(t209: F, t613: F, t617: F, t612: F) -> (F, F, F, F) {
    let t619 = t209 * t613 * t617;
    let t622 = 1.0 + t612 * t619 / 192.0;
    let t623 = f64::ln(t622);
    let t625 = 1.0 + 0.66725e-1 * t623;
    let t626 = 1.0 / t625;
    (t619, t622, t625, t626)
}
