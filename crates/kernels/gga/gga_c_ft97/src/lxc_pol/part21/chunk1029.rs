//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1029/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1029<F: Float>(t3076: F, t93046: F, t22514: F, t2258: F, t37481: F, t5585: F, t1608: F, t5584: F, t1611: F, t8: F, t5566: F, t7983: F, t92339: F, t401: F, t66: F, t22563: F) -> (F, F, F, F, F, F) {
    let t93047 = t3076 * t93046;
    let t93048 = t22514 * t2258;
    let t93053 = t5585 * t37481;
    let t93055 = t1608 * t5584 * t93053;
    let t93076 = t8 * t1611;
    let t93078 = t1608 * t5566 * t93076;
    let t93099 = t7983 * t92339;
    let t93102 = t401 * t66;
    let t93103 = t93102 * t22563;
    (t93047, t93048, t93055, t93078, t93099, t93103)
}
