//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 803/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk803<F: Float>(t30364: F, t3459: F, t1090: F, t1992: F, t30154: F, t7586: F, t1530: F, t7560: F, t14046: F, t2067: F) -> (F, F, F, F) {
    let t30365 = t30364 * t3459;
    let t30369 = t30154 * t7586 * t1992 * t1090;
    let t30371 = t1530 * t7560;
    let t30374 = t14046 * t2067;
    (t30365, t30369, t30371, t30374)
}
